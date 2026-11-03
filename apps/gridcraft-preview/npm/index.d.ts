/**
 * GridCraft Preview - Read-only XLSX/XLS spreadsheet viewer for the web
 *
 * @example
 * ```typescript
 * import init, { GridCraftPreview } from 'gridcraft-preview';
 *
 * // Initialize WASM module
 * await init();
 *
 * // Create preview instance
 * const preview = new GridCraftPreview();
 *
 * // Load a file
 * const fileInput = document.querySelector('input[type="file"]');
 * fileInput.addEventListener('change', async (e) => {
 *   const file = e.target.files[0];
 *   await preview.loadFile(file);
 * });
 *
 * // Control the view
 * preview.zoomIn();
 * preview.switchSheet('Sheet2');
 * preview.setLanguage('zh');
 * ```
 */

/**
 * Supported languages for the interface.
 */
export type Language =
  | 'en' // English
  | 'zh' // 简体中文
  | 'es' // Español
  | 'ja' // 日本語
  | 'ko' // 한국어
  | 'ru' // Русский
  | 'pt' // Português (Brasil)
  | 'de' // Deutsch
  | 'cs' // Čeština
  | 'fr' // Français
  | 'sr' // Српски
  | 'tr' // Türkçe
  | 'pl' // Polski
  | 'it' // Italiano
  | 'uk' // Українська
  | 'vi' // Tiếng Việt
  | 'nl' // Nederlands
  | 'ro' // Română
  | 'el'; // Ελληνικά

/**
 * Theme options.
 */
export type Theme = 'light' | 'dark' | 'auto';

/**
 * Find options.
 */
export interface FindOptions {
  /**
   * Whether to match case when searching.
   * @default false
   */
  matchCase?: boolean;
}

/**
 * GridCraft Preview main class.
 *
 * Provides a read-only spreadsheet viewer with features like:
 * - Opening XLSX/XLS files
 * - Sheet navigation
 * - Zoom controls
 * - Cell selection and viewing
 * - Search/find functionality
 * - Multi-language support
 * - Theme switching
 */
export class GridCraftPreview {
  /**
   * Create a new GridCraft Preview instance.
   *
   * @throws {Error} If WASM module is not initialized (call `init()` first)
   */
  constructor();

  /**
   * Load a spreadsheet file.
   *
   * @param file - File or Blob object containing the spreadsheet data
   * @param filename - Optional filename (auto-detected from File objects)
   * @returns Promise that resolves when the file is loaded
   * @throws {Error} If the file format is not supported or is corrupted
   *
   * @example
   * ```typescript
   * const file = await fetch('/sample.xlsx').then(r => r.blob());
   * await preview.loadFile(file, 'sample.xlsx');
   * ```
   */
  loadFile(file: File | Blob, filename?: string): Promise<void>;

  /**
   * Zoom in on the current sheet.
   */
  zoomIn(): void;

  /**
   * Zoom out on the current sheet.
   */
  zoomOut(): void;

  /**
   * Zoom to fit the entire sheet in the window.
   */
  zoomToFit(): void;

  /**
   * Set zoom to a specific percentage.
   *
   * @param percentage - Zoom level (e.g., 100 for 100%, 150 for 150%)
   * @example
   * ```typescript
   * preview.setZoom(125); // Set zoom to 125%
   * ```
   */
  setZoom(percentage: number): void;

  /**
   * Get names of all sheets in the workbook.
   *
   * @returns Array of sheet names
   */
  getSheetNames(): string[];

  /**
   * Get the name of the currently active sheet.
   *
   * @returns Current sheet name
   */
  getCurrentSheet(): string;

  /**
   * Switch to a different sheet.
   *
   * @param name - Sheet name to switch to
   * @throws {Error} If the sheet does not exist
   */
  switchSheet(name: string): void;

  /**
   * Get the current selection range.
   *
   * @returns Range reference (e.g., "A1", "B2:D5")
   */
  getSelection(): string;

  /**
   * Select a cell or range.
   *
   * @param range - Cell or range reference (e.g., "A1", "B2:D5")
   * @example
   * ```typescript
   * preview.selectRange('A1:C10');
   * ```
   */
  selectRange(range: string): void;

  /**
   * Find text in the current sheet.
   *
   * @param query - Text to search for
   * @param options - Search options
   * @returns Number of matches found
   */
  find(query: string, options?: FindOptions): number;

  /**
   * Find the next occurrence of the search query.
   *
   * @returns true if a match was found, false if no more matches
   */
  findNext(): boolean;

  /**
   * Find the previous occurrence of the search query.
   *
   * @returns true if a match was found, false if no more matches
   */
  findPrevious(): boolean;

  /**
   * Set the interface language.
   *
   * @param lang - Language code
   * @example
   * ```typescript
   * preview.setLanguage('zh'); // Switch to Chinese
   * preview.setLanguage('en'); // Switch back to English
   * ```
   */
  setLanguage(lang: Language): void;

  /**
   * Set the color theme.
   *
   * @param theme - Theme name ('light', 'dark', or 'auto' to follow system)
   * @example
   * ```typescript
   * preview.setTheme('dark');
   * preview.setTheme('auto'); // Follow system preference
   * ```
   */
  setTheme(theme: Theme): void;

  /** Configure whether the legacy Excel 97-2003 import notice is shown. */
  setShowXlsImportNotice(show: boolean): void;
}

/**
 * Initialize the WASM module.
 *
 * Must be called before creating any GridCraftPreview instances.
 *
 * @param input - Optional WASM module or path to WASM file
 * @returns Promise that resolves when initialization is complete
 *
 * @example
 * ```typescript
 * import init from 'gridcraft-preview';
 * await init();
 * ```
 */
export default function init(input?: string | URL | Request | Response | BufferSource | WebAssembly.Module | Promise<any>): Promise<void>;
