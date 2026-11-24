# GridCraft Preview

Read-only XLSX/XLS spreadsheet viewer for the web, powered by [GridCraft](https://getartcraft.com/apps/gridcraft).

## Features

- ✅ **Open XLSX/XLS files** from File/Blob objects
- ✅ **Sheet navigation** - Switch between worksheets
- ✅ **Zoom controls** - Zoom in/out/fit to window
- ✅ **Cell selection** - Select and view cell contents (read-only)
- ✅ **Search** - Find text in the current sheet
- ✅ **Copy** - Copy cell contents to clipboard
- ✅ **Multi-language** - 19 languages including English and Chinese
- ✅ **Theme switching** - Light, dark, or auto (follows system)
- ❌ **No editing** - All editing features are disabled

## Installation

```bash
npm install gridcraft-preview
```

## Usage

### Basic Example

```typescript
import init, { GridCraftPreview } from 'gridcraft-preview';

// Initialize WASM module (must be called once before using the preview)
await init();

// Create preview instance
const preview = new GridCraftPreview();

// Load a file
const fileInput = document.querySelector('input[type="file"]');
fileInput.addEventListener('change', async (e) => {
  const file = e.target.files[0];
  if (file) {
    await preview.loadFile(file);
  }
});

// Control the view
preview.zoomIn();
preview.zoomOut();
preview.zoomToFit();
preview.setZoom(125); // 125%

// Navigate sheets
const sheets = preview.getSheetNames();
console.log('Available sheets:', sheets);
preview.switchSheet('Sheet2');

// Search
const matchCount = preview.find('total', { matchCase: false });
console.log(`Found ${matchCount} matches`);
preview.findNext();

// Internationalization
preview.setLanguage('zh'); // Switch to Chinese
preview.setLanguage('en'); // Switch back to English

// Theme
preview.setTheme('dark');
preview.setTheme('auto'); // Follow system preference
```

### Loading from a URL

```typescript
import init, { GridCraftPreview } from 'gridcraft-preview';

await init();

const preview = new GridCraftPreview();

// Fetch and load a spreadsheet
const response = await fetch('/data/spreadsheet.xlsx');
const blob = await response.blob();
await preview.loadFile(blob, 'spreadsheet.xlsx');
```

### HTML Integration

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>GridCraft Preview Example</title>
</head>
<body>
  <div>
    <input type="file" id="fileInput" accept=".xlsx,.xls" />
    <button onclick="preview.zoomIn()">Zoom In</button>
    <button onclick="preview.zoomOut()">Zoom Out</button>
    <button onclick="preview.zoomToFit()">Fit</button>
  </div>
  
  <canvas id="gridcraft_canvas" style="width: 100%; height: 600px;"></canvas>

  <script type="module">
    import init, { GridCraftPreview } from './node_modules/gridcraft-preview/dist/index.js';

    await init();
    window.preview = new GridCraftPreview();

    document.getElementById('fileInput').addEventListener('change', async (e) => {
      const file = e.target.files[0];
      if (file) {
        await window.preview.loadFile(file);
      }
    });
  </script>
</body>
</html>
```

## API Reference

See [index.d.ts](./index.d.ts) for complete TypeScript definitions.

### `GridCraftPreview`

Main class for controlling the preview.

#### Constructor

```typescript
const preview = new GridCraftPreview();
```

#### Methods

##### File Operations

- `loadFile(file: File | Blob, filename?: string): Promise<void>` - Load a spreadsheet file

##### View Controls

- `zoomIn(): void` - Zoom in
- `zoomOut(): void` - Zoom out
- `zoomToFit(): void` - Zoom to fit window
- `setZoom(percentage: number): void` - Set specific zoom level

##### Sheet Navigation

- `getSheetNames(): string[]` - Get all sheet names
- `getCurrentSheet(): string` - Get current sheet name
- `switchSheet(name: string): void` - Switch to a sheet

##### Selection

- `getSelection(): string` - Get current selection range
- `selectRange(range: string): void` - Select a range

##### Search

- `find(query: string, options?: FindOptions): number` - Find text
- `findNext(): boolean` - Find next occurrence
- `findPrevious(): boolean` - Find previous occurrence

##### Localization

- `setLanguage(lang: Language): void` - Set interface language
- `setTheme(theme: Theme): void` - Set color theme

### Supported Languages

English (`en`), 简体中文 (`zh`), Español (`es`), 日本語 (`ja`), 한국어 (`ko`), Русский (`ru`), Português (`pt`), Deutsch (`de`), Čeština (`cs`), Français (`fr`), Српски (`sr`), Türkçe (`tr`), Polski (`pl`), Italiano (`it`), Українська (`uk`), Tiếng Việt (`vi`), Nederlands (`nl`), Română (`ro`), Ελληνικά (`el`)

### Supported File Formats

- **XLSX** - Excel 2007+ workbooks (full support)
- **XLSM** - Excel macro-enabled workbooks (viewing only, no macros)
- **XLS** - Excel 97-2003 workbooks (values and formulas, limited formatting)

## Browser Compatibility

Requires a modern browser with WebGPU or WebGL2 support:

- Chrome/Edge 79+
- Firefox 51+
- Safari 15+
- Opera 66+

## Size

- Uncompressed WASM: ~3 MB
- Gzipped: ~1 MB

## License

Dual-licensed under [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE), at your option.

## Credits

Built with [GridCraft](https://getartcraft.com/apps/gridcraft) - open-source spreadsheet software from the ArtCraft team.

## Links

- [GridCraft](https://getartcraft.com/apps/gridcraft)
- [GitHub Repository](https://github.com/storytold/gridcraft)
- [ArtCraft](https://getartcraft.com/)
- [Discord Community](https://discord.gg/artcraft)
