# GridCraft Preview

Read-only XLSX/XLS spreadsheet viewer for the web, built with GridCraft and compiled to WebAssembly.

## Overview

GridCraft Preview is a lightweight, read-only spreadsheet viewer that runs entirely in the browser. It's designed for embedding in web applications where you need to display Excel files without allowing editing.

### Features

✅ **Read-only viewing** - All editing commands are disabled at the engine level  
✅ **XLSX/XLS support** - Open Excel 2007+ and legacy 97-2003 files  
✅ **Sheet navigation** - Switch between multiple worksheets  
✅ **Zoom controls** - Zoom in, out, or fit to window  
✅ **Cell selection** - Select and view cell contents  
✅ **Search** - Find text in the current sheet  
✅ **Copy** - Copy cell contents to clipboard  
✅ **19 languages** - Including English, Chinese, Spanish, Japanese, and more  
✅ **Theme switching** - Light, dark, or follow system preference  
✅ **No backend required** - Runs entirely client-side  

### What's Disabled

❌ All editing (typing, pasting, formatting)  
❌ File save/export  
❌ Formula editing  
❌ Inserting/deleting rows, columns, sheets  
❌ Chart editing  
❌ Macro execution  

## Quick Start

### Development

1. **Install Prerequisites**

   ```bash
   # Install Rust and wasm32 target
   rustup target add wasm32-unknown-unknown

   # Install Trunk
   cargo install trunk
   ```

2. **Build and Run**

   ```bash
   cd apps/gridcraft-preview
   trunk serve --release
   ```

   Open http://127.0.0.1:8772

3. **Try the Example**

   The development server includes file upload and basic controls. Drop an XLSX file on the page or use the file input.

### NPM Package

See `npm/README.md` for full documentation on using the NPM package in your projects.

```bash
npm install gridcraft-preview
```

```javascript
import init, { GridCraftPreview } from 'gridcraft-preview';

await init();
const preview = new GridCraftPreview();
await preview.loadFile(fileBlob);
```

## Project Structure

```
apps/gridcraft-preview/
├── src/
│   ├── main.rs               # Entry point
│   ├── web.rs                # Browser shell and eframe setup
│   ├── preview_session.rs    # Read-only Session wrapper for embedders
│   ├── preview_ui.rs         # UI configuration
│   └── preview_api.rs        # JavaScript API (wasm-bindgen)
├── npm/
│   ├── package.json          # NPM package metadata
│   ├── index.d.ts            # TypeScript definitions
│   ├── README.md             # NPM package documentation
│   ├── example.html          # Complete example
│   └── scripts/
│       └── build.js          # Build script for NPM
├── Cargo.toml                # Rust dependencies
├── Trunk.toml                # Trunk build configuration
├── index.html                # Development HTML
├── DEPLOY.md                 # Deployment guide
└── README.md                 # This file
```

## Architecture

### Read-Only Enforcement

The preview mode is enforced at multiple levels:

1. **Engine read-only policy** (`Session::set_read_only`)
   - The preview shell enables the policy before it accepts files or commands
   - An allowlist keeps navigation, view, inspection and copy commands available
   - Editing commands are blocked at the engine boundary, including API and keyboard paths

2. **UI Configuration** (`preview_ui.rs`)
   - Hides File tab and edit ribbons
   - Disables formula bar editing
   - Removes save/export buttons
   - Collapses ribbon by default

3. **Services** (`preview_ui.rs`)
   - No file picker services
   - No download capability
   - Read-only clipboard (copy only)


## Building

### Development Build

```bash
trunk build --release
```

Output: `../../dist/preview/`

### NPM Package

```bash
cd npm
npm run build
```

This creates the distribution in `npm/dist/` with:
- WASM module
- JavaScript glue code
- TypeScript definitions
- Entry point

### Size

- Uncompressed WASM: ~3 MB
- Gzipped: ~1 MB

Size is optimized with:
- `wasm-opt -Oz` (via Trunk)
- Link-time optimization (LTO)
- Strip debug symbols
- Minimal dependencies

## Testing

### Manual Testing

1. Build and serve: `trunk serve --release`
2. Open http://127.0.0.1:8772
3. Upload test XLSX files
4. Verify features work:
   - File loading
   - Sheet switching
   - Zoom controls
   - Language switching
   - Theme switching
   - Search/find
   - Copy to clipboard
   - **Verify editing is disabled** - try typing in cells

### Automated Testing

```bash
# Check compilation for wasm32
cargo check -p gridcraft-preview --target wasm32-unknown-unknown

# Run workspace tests (engine and UI layer)
cargo test -p gridcraft-engine
cargo test -p gridcraft-ui-egui
```

## Deployment

See `DEPLOY.md` for complete deployment instructions including:
- Prerequisites
- Building for production
- Publishing to NPM
- CI/CD automation
- Size optimization
- Troubleshooting

## Browser Compatibility

Requires WebGPU or WebGL2:

| Browser | Minimum Version |
|---------|----------------|
| Chrome/Edge | 79+ |
| Firefox | 51+ |
| Safari | 15+ |
| Opera | 66+ |

## License

Dual-licensed under MIT or Apache-2.0, at your option. Same as the main GridCraft project.

- MIT: See `LICENSE-MIT`
- Apache-2.0: See `LICENSE-APACHE`

## Links

- **GridCraft**: https://getartcraft.com/apps/gridcraft
- **GitHub**: https://github.com/storytold/gridcraft
- **Discord**: https://discord.gg/artcraft
- **NPM Package**: `gridcraft-preview` (after publication)

## Contributing

GridCraft Preview is part of the GridCraft project. Contributions welcome!

1. Check out the feature branch: `feature/gridcraft-preview`
2. Make your changes
3. Test with `trunk serve` and `npm run build`
4. Submit a PR against the feature branch

## Support

- GitHub Issues: https://github.com/storytold/gridcraft/issues
- Discord Community: https://discord.gg/artcraft
- Tag issues with `preview` label

## Roadmap

Current status: **Alpha** - Core functionality complete, ready for testing.

### Completed

- ✅ Basic WASM build infrastructure
- ✅ Command filtering for read-only mode
- ✅ UI configuration
- ✅ JavaScript API skeleton
- ✅ NPM packaging
- ✅ TypeScript definitions
- ✅ Example page
- ✅ Documentation

### TODO

- [ ] Actually wire up JavaScript API to running app
- [ ] Implement inbox for async file loading
- [ ] Add event system (onFileLoaded, onSheetChanged, etc.)
- [ ] Improve file loading feedback
- [ ] Add more API methods (getCell, getCellValue, etc.)
- [ ] Performance testing with large files
- [ ] Browser compatibility testing
- [ ] Published NPM package
- [ ] CDN hosting option
- [ ] React/Vue/Svelte wrapper components

### Known Limitations

1. **JavaScript API is not fully wired** - The preview runs but the API methods don't connect to the running app yet. This needs a proper communication channel between JS and the running eframe app.

2. **File loading via API** - Currently files can be dropped on the window, but `loadFile()` API doesn't send files to the inbox yet.

3. **No events** - The on/off event system is not implemented.

4. **CJK fonts** - Chinese, Japanese, Korean text may not render well without system fonts. The web build doesn't bundle fonts by default.

## Credits

Built with:
- [GridCraft](https://github.com/storytold/gridcraft) - The underlying spreadsheet engine
- [egui](https://github.com/emilk/egui) - Immediate mode GUI
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) - egui framework
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/WASM/JS bindings
- [Trunk](https://trunkrs.dev/) - WASM build tool

Made by the [ArtCraft](https://getartcraft.com/) team and community.
