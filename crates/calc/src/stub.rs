//! Lightweight calculation stub when `eval` is disabled (read-only preview builds).
//!
//! Cached cell values in the workbook are displayed as saved, and chart series references
//! (`Sheet1!A1:A10`) still resolve directly from the cached sheet cells without pulling in the
//! worksheet function library or regex engine.

use gridcraft_core::{Array, CellRef, MAX_COLS, MAX_ROWS, RangeRef, Value};
use gridcraft_model::Workbook;

pub mod eval {
    use gridcraft_core::RangeRef;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Area {
        pub sheet: usize,
        pub range: RangeRef,
    }

    pub const SPECIAL_FUNCTIONS: &[&str] = &[];

    pub fn is_known_function(_name: &str) -> bool {
        false
    }
}

#[derive(Default)]
pub struct DisplayedPrecision;

impl DisplayedPrecision {
    pub fn new() -> Self {
        Self
    }

    pub fn round(&mut self, _wb: &Workbook, _sheet: usize, _c: CellRef, n: f64) -> f64 {
        n
    }

    pub fn round_value(&mut self, _wb: &Workbook, _sheet: usize, _c: CellRef, v: Value) -> Value {
        v
    }
}

pub mod recalc {
    use gridcraft_core::{CellRef, RangeRef, Value};
    use gridcraft_formula::Expr;
    use gridcraft_model::Workbook;

    pub type Key = (usize, CellRef);

    #[derive(Clone, Debug, Default)]
    pub struct Graph;

    impl Graph {
        pub fn len(&self) -> usize {
            0
        }

        pub fn is_empty(&self) -> bool {
            true
        }

        pub fn dependents(&self, _sheet: usize, _c: CellRef, _out: &mut Vec<Key>) {}

        pub fn dependents_of_range(&self, _sheet: usize, _range: RangeRef, _out: &mut Vec<Key>) {}

        pub fn precedents_of(&self, _k: Key) -> Vec<super::eval::Area> {
            Vec::new()
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Calc {
        pub graph: Graph,
        pub fixed_now: Option<f64>,
        pub circular: Vec<Key>,
        pub last_recalc_cells: usize,
    }

    impl Calc {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn rebuild(&mut self, _wb: &Workbook) {}

        pub fn recalc_all(&mut self, _wb: &mut Workbook) {}

        pub fn cells_changed(&mut self, _wb: &mut Workbook, _changed: &[Key]) {}
    }

    #[derive(Clone, Debug)]
    pub enum Bounded {
        Areas(Vec<super::eval::Area>),
        Value(Value),
        Exhausted,
    }

    pub fn now_serial() -> f64 {
        gridcraft_core::date::unix_now_ms().map_or(46302.5, gridcraft_core::date::serial_from_unix_ms)
    }

    pub fn evaluate(wb: &Workbook, sheet: usize, _at: CellRef, formula: &str) -> Value {
        super::resolve_cached_formula(wb, sheet, formula)
    }

    pub fn evaluate_expr(_wb: &Workbook, _sheet: usize, _at: CellRef, _expr: &Expr) -> Value {
        Value::Empty
    }

    pub fn evaluate_bounded(wb: &Workbook, sheet: usize, at: CellRef, formula: &str, _work: u64) -> Bounded {
        Bounded::Value(evaluate(wb, sheet, at, formula))
    }
}

pub use recalc::{Bounded, Calc, Key, evaluate, evaluate_bounded, now_serial};

const FULL_SIZE_CELLS: u64 = 16_384;
const MAX_PREVIEW_RANGE_CELLS: u64 = 1_000_000;

fn resolve_cached_formula(wb: &Workbook, sheet: usize, formula: &str) -> Value {
    let f = formula.trim().trim_start_matches('=').trim();
    if f.is_empty() {
        return Value::Empty;
    }
    if let Some(quoted) = f.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return Value::Text(quoted.replace("\"\"", "\"").into());
    }
    if f.eq_ignore_ascii_case("TRUE") {
        return Value::Bool(true);
    }
    if f.eq_ignore_ascii_case("FALSE") {
        return Value::Bool(false);
    }
    if let Ok(n) = f.parse::<f64>()
        && n.is_finite()
    {
        return Value::Number(n);
    }
    let Some((si, mut r)) = parse_sheet_range(wb, sheet, f) else {
        return Value::Empty;
    };
    let Some(sh) = wb.sheet(si) else {
        return Value::Empty;
    };
    let cells = u64::from(r.height()).saturating_mul(u64::from(r.width()));
    let to_edge = r.end.row == MAX_ROWS - 1 || r.end.col == MAX_COLS - 1;
    if cells > FULL_SIZE_CELLS && (to_edge || cells > MAX_PREVIEW_RANGE_CELLS) {
        let end = match sh.used_range() {
            Some(u) => CellRef::new(r.end.row.min(u.end.row.max(r.start.row)), r.end.col.min(u.end.col.max(r.start.col))),
            None => r.start,
        };
        r = RangeRef::new(r.start, end);
    }
    if r.is_single() {
        return sh.cell(r.start).map_or(Value::Empty, |c| c.value.clone());
    }
    let total = u64::from(r.height()).saturating_mul(u64::from(r.width()));
    if total > MAX_PREVIEW_RANGE_CELLS {
        return Value::Empty;
    }
    let rows = r.height() as usize;
    let cols = r.width() as usize;
    let mut data = Vec::with_capacity(rows.saturating_mul(cols));
    for row in r.start.row..=r.end.row {
        for col in r.start.col..=r.end.col {
            data.push(sh.cell(CellRef::new(row, col)).map_or(Value::Empty, |c| c.value.clone()));
        }
    }
    Value::Array(Array { rows, cols, data }.into())
}

fn parse_sheet_range(wb: &Workbook, sheet: usize, f: &str) -> Option<(usize, RangeRef)> {
    let (si, addr) = match f.rfind('!') {
        Some(i) => {
            let name = gridcraft_core::unquote_sheet(f.get(..i)?.trim());
            (wb.sheet_index(&name)?, f.get(i + 1..)?)
        }
        None => (sheet, f),
    };
    RangeRef::parse(addr).map(|r| (si, r))
}
