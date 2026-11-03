# GridCraft Preview - Deployment Guide

Complete guide to building and publishing the gridcraft-preview NPM package.

## Prerequisites

### 1. Install Rust and Cargo

```bash
# If not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install wasm32 Target

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Install Trunk

```bash
cargo install trunk
```

### 4. Install wasm-bindgen-cli (Optional, Trunk includes it)

```bash
cargo install wasm-bindgen-cli
```

## Development Build

### 1. Build the WASM Module

From the `apps/gridcraft-preview` directory:

```bash
cd apps/gridcraft-preview
trunk build --release
```

Output will be in `../../dist/preview/`.

### 2. Test Locally

Start a development server:

```bash
trunk serve --release
```

Then open http://127.0.0.1:8772 in your browser.

Or use a simple HTTP server:

```bash
# Python 3
cd ../../dist/preview
python3 -m http.server 8000

# Node.js
npx http-server -p 8000
```

### 3. Test the Example

```bash
cd npm
# First build the package
node scripts/build.js

# Then serve the example
npx http-server -p 8080
```

Open http://localhost:8080/example.html

## NPM Package Build

### 1. Build for NPM

From the `npm` directory:

```bash
cd apps/gridcraft-preview/npm
npm run build
```

This script:
- Checks for required tools (trunk, wasm32 target)
- Builds the WASM module with Trunk
- Copies output files to `npm/dist/`
- Copies license files
- Creates the entry point

### 2. Verify the Package

```bash
# Check package contents
npm pack --dry-run

# Create a tarball
npm pack
```

This creates `gridcraft-preview-0.1.0.tgz`.

### 3. Test the Package Locally

In another project:

```bash
npm install /path/to/gridcraft-preview-0.1.0.tgz
```

Or link it:

```bash
# In the npm directory
npm link

# In your test project
npm link gridcraft-preview
```

## Publishing to NPM

### 1. Update Version

Edit `npm/package.json` and bump the version:

```json
{
  "version": "0.1.0" // → "0.1.1", "0.2.0", etc.
}
```

### 2. Login to NPM

```bash
npm login
```

### 3. Publish

```bash
# Dry run to check what will be published
npm publish --dry-run

# Actual publish
npm publish

# Or publish as beta/alpha
npm publish --tag beta
```

### 4. Verify Publication

```bash
# Check on NPM
npm view gridcraft-preview

# Install in a test project
npm install gridcraft-preview
```

## CI/CD Automation

### GitHub Actions Example

Create `.github/workflows/publish-preview.yml`:

```yaml
name: Publish GridCraft Preview

on:
  push:
    tags:
      - 'preview-v*'

jobs:
  build-and-publish:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown

      - name: Install Trunk
        run: cargo install trunk

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'

      - name: Build NPM Package
        run: |
          cd apps/gridcraft-preview/npm
          npm run build

      - name: Publish to NPM
        run: |
          cd apps/gridcraft-preview/npm
          npm publish
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

## File Size Optimization

### Current Sizes

- Uncompressed WASM: ~3-4 MB
- Gzipped: ~1-1.5 MB

### Optimization Tips

1. **Trunk already uses wasm-opt with -Oz**
   - Configured in `index.html`: `data-wasm-opt="z"`

2. **Additional Cargo optimizations** (in workspace `Cargo.toml`):

```toml
[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
panic = 'abort'     # Smaller binary
strip = true        # Remove debug symbols
```

3. **Feature flags** to exclude unused code:

```toml
# In gridcraft-preview/Cargo.toml
[features]
default = []
# Could add feature flags to exclude unused UI components
```

4. **Server Configuration**:
   - Enable gzip/brotli compression
   - Set proper MIME types
   - Cache the WASM file aggressively

## Troubleshooting

### "Trunk not found"

```bash
cargo install trunk
# Or
cargo install --git https://github.com/trunk-rs/trunk trunk
```

### "wasm32-unknown-unknown target not installed"

```bash
rustup target add wasm32-unknown-unknown
```

### "WASM file too large"

1. Check that release mode is enabled: `trunk build --release`
2. Verify wasm-opt is running: look for "Optimizing wasm" in build output
3. Check workspace `Cargo.toml` for size optimizations

### "Module initialization failed"

- Make sure you're calling `await init()` before creating `GridCraftPreview`
- Check browser console for specific error messages
- Verify the WASM file is being served with correct MIME type (`application/wasm`)

### "Cannot find module"

- Ensure all files are copied to `npm/dist/`:
  - `gridcraft-preview-*.js`
  - `gridcraft-preview-*_bg.wasm`
  - `index.js` (entry point)
  - `index.d.ts` (TypeScript definitions)

## Versioning Strategy

Follow semantic versioning (semver):

- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.2.0): New features, backward compatible
- **PATCH** (0.1.1): Bug fixes, backward compatible

Recommended versioning during development:

- `0.1.0` - Initial release
- `0.1.x` - Bug fixes
- `0.2.0` - Add new API methods
- `1.0.0` - Stable release, API locked

## Distribution Checklist

Before publishing:

- [ ] Version bumped in `package.json`
- [ ] CHANGELOG.md updated
- [ ] README.md is current
- [ ] TypeScript definitions are complete
- [ ] Example works
- [ ] Tested in clean project with `npm install`
- [ ] File size is reasonable (<2 MB gzipped)
- [ ] License files included
- [ ] Git tag created for the version

## Post-Publication

After publishing:

1. **Create GitHub Release**
   ```bash
   git tag -a preview-v0.1.0 -m "GridCraft Preview v0.1.0"
   git push origin preview-v0.1.0
   ```

2. **Update Documentation**
   - README with installation instructions
   - API documentation
   - Examples repository

3. **Announce**
   - Discord community
   - Project README
   - Release notes

## Support

- GitHub Issues: https://github.com/storytold/gridcraft/issues
- Discord: https://discord.gg/artcraft
- Docs: https://getartcraft.com/apps/gridcraft
