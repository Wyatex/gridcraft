# GridCraft Preview - Usage Guide

Complete guide for using GridCraft Preview in your projects.

## Installation

### Option 1: NPM Package (After Publication)

```bash
npm install gridcraft-preview
```

### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/storytold/gridcraft.git
cd gridcraft

# Checkout the feature branch
git checkout feature/gridcraft-preview

# Install prerequisites
rustup target add wasm32-unknown-unknown
cargo install trunk

# Build
cd apps/gridcraft-preview
trunk build --release

# Output will be in ../../dist/preview/
```

## Development Setup

### 1. Start Development Server

```bash
cd apps/gridcraft-preview
trunk serve --release
```

Open http://127.0.0.1:8772

### 2. Test with Your Files

- Drop XLSX/XLS files onto the page
- Or use the file input control
- Test zoom, sheet navigation, language switching

## Using in Your Project

### Basic Integration

```html
<!DOCTYPE html>
<html>
<head>
  <title>My App with GridCraft Preview</title>
  <style>
    #gridcraft_canvas {
      width: 100%;
      height: 600px;
      border: 1px solid #ddd;
    }
  </style>
</head>
<body>
  <h1>Spreadsheet Viewer</h1>
  <input type="file" id="fileInput" accept=".xlsx,.xls" />
  
  <canvas id="gridcraft_canvas"></canvas>
  <div id="gridcraft_loading">Loading...</div>

  <script type="module">
    import init, { GridCraftPreview } from './gridcraft-preview/index.js';

    // Initialize WASM module
    await init();

    // Create preview instance
    const preview = new GridCraftPreview();

    // Load file
    document.getElementById('fileInput').addEventListener('change', async (e) => {
      const file = e.target.files[0];
      if (file) {
        try {
          await preview.loadFile(file);
          console.log('File loaded!');
        } catch (err) {
          console.error('Failed to load:', err);
        }
      }
    });
  </script>
</body>
</html>
```

### With Controls

```html
<div class="controls">
  <button onclick="preview.zoomIn()">Zoom In</button>
  <button onclick="preview.zoomOut()">Zoom Out</button>
  <button onclick="preview.zoomToFit()">Fit Window</button>
  
  <select onchange="preview.setLanguage(this.value)">
    <option value="en">English</option>
    <option value="zh">中文</option>
    <option value="es">Español</option>
  </select>
  
  <select onchange="preview.setTheme(this.value)">
    <option value="auto">Auto</option>
    <option value="light">Light</option>
    <option value="dark">Dark</option>
  </select>
</div>

<script type="module">
  // Make preview global for onclick handlers
  window.preview = new GridCraftPreview();
</script>
```

### Load from URL

```javascript
// Fetch a file from your server
const response = await fetch('/api/files/spreadsheet.xlsx');
const blob = await response.blob();
await preview.loadFile(blob, 'spreadsheet.xlsx');
```

### React Integration

```tsx
import { useEffect, useRef, useState } from 'react';
import init, { GridCraftPreview } from 'gridcraft-preview';

function SpreadsheetViewer() {
  const [preview, setPreview] = useState<GridCraftPreview | null>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    // Initialize WASM module and create preview
    init().then(() => {
      const p = new GridCraftPreview();
      setPreview(p);
    });
  }, []);

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file && preview) {
      try {
        await preview.loadFile(file);
      } catch (err) {
        console.error('Failed to load file:', err);
      }
    }
  };

  return (
    <div>
      <input type="file" onChange={handleFileChange} accept=".xlsx,.xls" />
      
      <div className="controls">
        <button onClick={() => preview?.zoomIn()}>+</button>
        <button onClick={() => preview?.zoomOut()}>-</button>
        <button onClick={() => preview?.zoomToFit()}>Fit</button>
      </div>

      <canvas ref={canvasRef} id="gridcraft_canvas" />
      <div id="gridcraft_loading">Loading GridCraft Preview...</div>
    </div>
  );
}
```

### Vue Integration

```vue
<template>
  <div>
    <input type="file" @change="handleFile" accept=".xlsx,.xls" />
    
    <div class="controls">
      <button @click="zoomIn">+</button>
      <button @click="zoomOut">-</button>
      <button @click="zoomFit">Fit</button>
    </div>

    <canvas id="gridcraft_canvas"></canvas>
    <div id="gridcraft_loading">Loading...</div>
  </div>
</template>

<script>
import init, { GridCraftPreview } from 'gridcraft-preview';

export default {
  data() {
    return {
      preview: null
    };
  },

  async mounted() {
    await init();
    this.preview = new GridCraftPreview();
  },

  methods: {
    async handleFile(event) {
      const file = event.target.files[0];
      if (file && this.preview) {
        try {
          await this.preview.loadFile(file);
        } catch (err) {
          console.error('Failed to load:', err);
        }
      }
    },

    zoomIn() { this.preview?.zoomIn(); },
    zoomOut() { this.preview?.zoomOut(); },
    zoomFit() { this.preview?.zoomToFit(); }
  }
};
</script>
```

## API Reference

### Initialization

```javascript
import init from 'gridcraft-preview';
await init(); // Must be called before creating GridCraftPreview
```

### Creating Preview

```javascript
import { GridCraftPreview } from 'gridcraft-preview';
const preview = new GridCraftPreview();
```

### Loading Files

```javascript
// From File input
await preview.loadFile(file);

// From Blob with custom name
await preview.loadFile(blob, 'my-spreadsheet.xlsx');

// From fetch
const response = await fetch(url);
const blob = await response.blob();
await preview.loadFile(blob);
```

### Zoom Controls

```javascript
preview.zoomIn();         // Zoom in one step
preview.zoomOut();        // Zoom out one step
preview.zoomToFit();      // Fit entire sheet in window
preview.setZoom(150);     // Set zoom to 150%
```

### Sheet Navigation

```javascript
const sheets = preview.getSheetNames();  // ['Sheet1', 'Sheet2', ...]
const current = preview.getCurrentSheet();  // 'Sheet1'
preview.switchSheet('Sheet2');  // Switch to Sheet2
```

### Selection

```javascript
const range = preview.getSelection();  // 'A1:C5'
preview.selectRange('B2:D10');  // Select range
```

### Search

```javascript
// Find text (returns match count)
const count = preview.find('total', { matchCase: false });

// Navigate matches
preview.findNext();      // Go to next match
preview.findPrevious();  // Go to previous match
```

### Localization

```javascript
// Set language
preview.setLanguage('zh');  // Chinese
preview.setLanguage('en');  // English
preview.setLanguage('es');  // Spanish
preview.setLanguage('ja');  // Japanese

// Supported: en, zh, es, ja, ko, ru, pt, de, cs, fr, sr, tr, pl, it, uk, vi, nl, ro, el
```

### Theme

```javascript
preview.setTheme('light');  // Light theme
preview.setTheme('dark');   // Dark theme
preview.setTheme('auto');   // Follow system preference
```

## Styling

### Canvas Size

The canvas fills its container. Style it with CSS:

```css
#gridcraft_canvas {
  width: 100%;
  height: 600px; /* Fixed height */
  /* Or */
  height: 100vh; /* Full viewport */
  /* Or */
  height: calc(100vh - 120px); /* With header/footer */
}
```

### Loading Screen

Customize the loading indicator:

```html
<div id="gridcraft_loading" class="my-loading">
  <div class="spinner"></div>
  <p>Loading spreadsheet...</p>
</div>

<style>
.my-loading {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid #f3f3f3;
  border-top: 3px solid #1f9d55;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>
```

The loading element is automatically hidden once GridCraft initializes.

## Error Handling

```javascript
try {
  await preview.loadFile(file);
} catch (err) {
  if (err.message.includes('format')) {
    console.error('Unsupported file format');
  } else if (err.message.includes('corrupted')) {
    console.error('File is corrupted');
  } else {
    console.error('Failed to load file:', err);
  }
}
```

## Performance Tips

### Large Files

- The viewer uses virtual scrolling for large sheets
- Recalculation is incremental
- Only visible cells are rendered

### Memory

- Close files when done to free memory
- Consider limiting concurrent preview instances

### Bundle Size

- Compressed WASM is ~1 MB
- Use code splitting to load on demand
- Consider lazy loading the preview component

Example with dynamic import:

```javascript
async function showPreview(file) {
  // Load GridCraft Preview only when needed
  const { default: init, GridCraftPreview } = await import('gridcraft-preview');
  
  await init();
  const preview = new GridCraftPreview();
  await preview.loadFile(file);
}
```

## Troubleshooting

### "Failed to initialize"

- Make sure you called `await init()` before creating `GridCraftPreview`
- Check browser console for specific error
- Verify WASM file is being served correctly

### "Cannot find module"

- Ensure all files are copied from the build:
  - `gridcraft-preview-*.js`
  - `gridcraft-preview-*_bg.wasm`
  - `index.js`
- Check that your bundler handles WASM files

### Blank Canvas

- Make sure the canvas has explicit dimensions (width/height)
- Check that GridCraft Preview loaded successfully
- Verify file was loaded without errors

### File Won't Load

- Check file format is XLSX or XLS
- Verify file is not corrupted
- Check browser console for errors
- Some very old XLS files may have limited support

## Browser Requirements

- Chrome 79+
- Firefox 51+
- Safari 15+
- Edge 79+
- Opera 66+

Must support WebGPU or WebGL2.

## Security Considerations

- GridCraft Preview runs entirely client-side
- Files are processed in the browser, never sent to a server
- No network requests after WASM module loads
- Safe to use with sensitive spreadsheets

## Examples

See `npm/example.html` for a complete working example with:
- File upload
- Drag and drop
- All controls
- Error handling
- Status messages
- Keyboard shortcuts

## Support

- Issues: https://github.com/storytold/gridcraft/issues
- Discord: https://discord.gg/artcraft
- Email: support@getartcraft.com (coming soon)

## Next Steps

- Read `DEPLOY.md` for build and publication instructions
- Check `README.md` for architecture details
- See `npm/README.md` for NPM package documentation
- Join Discord for community support
