//! PivotTables: pivot cache definitions and records (ECMA-376 Part 1 §18.10.1) and PivotTable
//! definitions (§18.10.1.73), written from the model's [`PivotTable`]s and read back into them.
//!
//! Writing: one cache per PivotTable, with a records snapshot of the source list (omitted, with
//! `saveData="0"`, for very large sources) and `refreshOnLoad="1"` so a reader recomputes the
//! report from the source on open. Fields used on an axis get shared items (or, for date
//! grouping, `fieldGroup` group items) so the table's `<items>` can refer to them. The report
//! cells the engine wrote stay on the sheet as plain cells.
//!
//! Distinct Count: SpreadsheetML's subtotal list has no distinct count (Excel has one only for
//! PivotTables on its Data Model, whose binary model part we don't write). Such a value field is
//! written as `subtotal="count"` with a GridCraft `extLst` marker ([`EXT_GC_DATA_FIELD`]) that
//! GridCraft reads back as Distinct Count; other readers ignore the unknown extension. The cache
//! of a PivotTable with a Distinct Count field is written without `refreshOnLoad`, so Excel opens
//! it showing the distinct counts already on the sheet. A Refresh in Excel recomputes the field as
//! Count.
//!
//! Reading: the cache's source and field names plus the table's areas, functions, show-values-as,
//! layout flags, grand totals and style become a model [`PivotTable`]. Anything we don't model
//! (OLAP sources, calculated fields, numeric grouping…) becomes a warning.

use std::collections::HashMap;
#[cfg(feature = "write")]
use std::fmt::Write as _;

#[cfg(feature = "write")]
use gridcraft_core::date::datetime_from_serial;
#[cfg(feature = "write")]
use gridcraft_core::Value;
use gridcraft_core::{CellRef, DateSystem, RangeRef, number_to_text};
use gridcraft_formula::quote_sheet;
#[cfg(feature = "write")]
use gridcraft_model::Workbook;
use gridcraft_model::{
    PivotCalcField, PivotDateGroup, PivotField, PivotFilter, PivotFunc, PivotLayout, PivotShowAs, PivotSort, PivotTable, PivotValue,
};

use crate::IoError;
use crate::read::Ctx;
#[cfg(feature = "write")]
use crate::write::{NS_MAIN, NS_REL, Out, Rels, XML_DECL};
#[cfg(feature = "write")]
use crate::xml::esc_attr;
use crate::xml::{El, abs_range};

mod read;
#[cfg(feature = "write")]
mod write;

pub use read::{CachesIn, read_pivots};
#[cfg(feature = "write")]
pub use write::PivotWriter;

#[cfg(feature = "write")]
pub const CT_PIVOT_TABLE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml";
#[cfg(feature = "write")]
pub const CT_PIVOT_CACHE_DEF: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml";
#[cfg(feature = "write")]
pub const CT_PIVOT_CACHE_REC: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml";

/// Largest source block scanned for a cache (cells); bigger sources are not written.
#[cfg(feature = "write")]
const MAX_SOURCE_CELLS: u64 = 10_000_000;
/// Largest records snapshot written (cells); bigger caches are written without records.
#[cfg(feature = "write")]
const MAX_RECORD_CELLS: u64 = 2_000_000;
/// Most shared items kept per field (Excel's limit is 1,048,576 unique items).
#[cfg(feature = "write")]
const MAX_ITEMS: usize = 1_048_576;
/// Most cache fields read.
const MAX_FIELDS: usize = 16_384;
/// The `x14:dataField` extension (Excel 2010 show-values-as additions such as rank).
#[cfg(feature = "write")]
const EXT_X14_DATA_FIELD: &str = "{E15A36E0-9728-4e99-A89B-3F7291B0FE68}";
#[cfg(feature = "write")]
const EXT_X14_PIVOT_TABLE: &str = "{962EF5D1-5CA2-4c93-8EF4-DBF5C05439D2}";
#[cfg(feature = "write")]
const NS_X14: &str = "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main";
/// GridCraft's `dataField` extension: `<gc:summary func="distinctCount"/>` marks a value field
/// written as `subtotal="count"` as a Distinct Count (see the module documentation).
const EXT_GC_DATA_FIELD: &str = "{80B1AD42-0C9F-45FF-B109-D57FB0F4F0B4}";
#[cfg(feature = "write")]
const NS_GC: &str = "https://github.com/storytold/gridcraft/ns/pivot/2026";

#[cfg(feature = "write")]
const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

// ================================================================ shared helpers

/// The pivot item label of a value (blank shows as "(blank)").
#[cfg(feature = "write")]
fn label(v: &Value) -> String {
    match v {
        Value::Empty => "(blank)".into(),
        Value::Number(n) => number_to_text(*n),
        Value::Text(t) => t.to_string(),
        Value::Bool(b) => if *b { "TRUE" } else { "FALSE" }.into(),
        Value::Error(e) => e.as_str().into(),
        Value::Array(_) => label(&v.scalar()),
    }
}

#[cfg(feature = "write")]
fn same(a: &str, b: &str) -> bool {
    a.to_lowercase() == b.to_lowercase()
}

#[cfg(feature = "write")]
fn func_name(f: PivotFunc) -> &'static str {
    match f {
        PivotFunc::Sum => "sum",
        PivotFunc::Count => "count",
        PivotFunc::Average => "average",
        PivotFunc::Max => "max",
        PivotFunc::Min => "min",
        PivotFunc::Product => "product",
        PivotFunc::CountNumbers => "countNums",
        // The file format's subtotal list has no distinct count (Excel keeps it in a Data Model
        // extension), so the nearest function is written, with a GridCraft extension that marks
        // the field as Distinct Count (see the module documentation).
        PivotFunc::DistinctCount => "count",
        PivotFunc::StdDev => "stdDev",
        PivotFunc::StdDevP => "stdDevp",
        PivotFunc::Var => "var",
        PivotFunc::VarP => "varp",
    }
}

fn func_from(s: &str) -> Option<PivotFunc> {
    Some(match s {
        "sum" => PivotFunc::Sum,
        "count" => PivotFunc::Count,
        "average" => PivotFunc::Average,
        "max" => PivotFunc::Max,
        "min" => PivotFunc::Min,
        "product" => PivotFunc::Product,
        "countNums" => PivotFunc::CountNumbers,
        "stdDev" => PivotFunc::StdDev,
        "stdDevp" => PivotFunc::StdDevP,
        "var" => PivotFunc::Var,
        "varp" => PivotFunc::VarP,
        _ => return None,
    })
}

fn func_caption(f: PivotFunc) -> &'static str {
    match f {
        PivotFunc::Sum => "Sum",
        PivotFunc::Count => "Count",
        PivotFunc::Average => "Average",
        PivotFunc::Max => "Max",
        PivotFunc::Min => "Min",
        PivotFunc::Product => "Product",
        PivotFunc::CountNumbers => "Count",
        PivotFunc::DistinctCount => "Distinct Count",
        PivotFunc::StdDev => "StdDev",
        PivotFunc::StdDevP => "StdDevp",
        PivotFunc::Var => "Var",
        PivotFunc::VarP => "Varp",
    }
}

#[cfg(feature = "write")]
fn group_by_name(g: PivotDateGroup) -> &'static str {
    match g {
        PivotDateGroup::Years => "years",
        PivotDateGroup::Quarters => "quarters",
        PivotDateGroup::Months => "months",
        PivotDateGroup::Days => "days",
        PivotDateGroup::None => "range",
    }
}

#[cfg(feature = "write")]
fn group_caption(g: PivotDateGroup) -> &'static str {
    match g {
        PivotDateGroup::Years => "Years",
        PivotDateGroup::Quarters => "Quarters",
        PivotDateGroup::Months => "Months",
        PivotDateGroup::Days => "Days",
        PivotDateGroup::None => "Group",
    }
}

/// `2024-03-05T00:00:00` for a date serial.
#[cfg(feature = "write")]
fn iso(sys: DateSystem, serial: f64) -> Option<String> {
    let d = datetime_from_serial(sys, serial)?;
    Some(format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", d.year, d.month, d.day, d.hour, d.minute, d.second))
}

/// `3/5/2024` for a date serial (group boundary captions).
#[cfg(feature = "write")]
fn mdy(sys: DateSystem, serial: f64) -> String {
    match datetime_from_serial(sys, serial) {
        Some(d) => format!("{}/{}/{}", d.month, d.day, d.year),
        None => number_to_text(serial),
    }
}

// ================================================================ writing

/// Identity of a shared item (text items match case-insensitively, as in Excel).
#[cfg(feature = "write")]
#[derive(Clone, PartialEq, Eq, Hash)]
enum Key {
    Blank,
    Num(u64),
    Text(String),
    Bool(bool),
    Err(&'static str),
}

#[cfg(feature = "write")]
fn key(v: &Value) -> Key {
    match v {
        Value::Empty => Key::Blank,
        Value::Number(n) => Key::Num(if *n == 0.0 { 0 } else { n.to_bits() }),
        Value::Text(t) => Key::Text(t.to_lowercase()),
        Value::Bool(b) => Key::Bool(*b),
        Value::Error(e) => Key::Err(e.as_str()),
        Value::Array(_) => key(&v.scalar()),
    }
}
