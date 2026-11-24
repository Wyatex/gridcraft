#!/usr/bin/env node

/**
 * Build script for gridcraft-preview NPM package.
 *
 * This script:
 * 1. Checks that Trunk and wasm-bindgen are installed
 * 2. Builds the WASM module with Trunk
 * 3. Copies the output files to npm/dist/
 * 4. Copies license files
 */

const fs = require('fs');
const path = require('path');
const zlib = require('zlib');
const { execSync } = require('child_process');

const ROOT = path.resolve(__dirname, '../..');
const NPM_DIR = path.resolve(__dirname, '..');
const DIST_DIR = path.join(NPM_DIR, 'dist');
const BUILD_OUTPUT = path.join(ROOT, '../../dist/preview');

console.log('GridCraft Preview - Build Script');
console.log('=================================\n');

// Check for required tools
function checkTool(command, name, installHint) {
  try {
    execSync(`${command} --version`, { stdio: 'ignore' });
    console.log(`✓ ${name} is installed`);
    return true;
  } catch (e) {
    console.error(`✗ ${name} is not installed`);
    console.error(`  Install with: ${installHint}`);
    return false;
  }
}

console.log('Checking dependencies...');
const hasTrunk = checkTool('trunk', 'Trunk', 'cargo install trunk');
const hasWasmTarget = true; // We'll check this differently

if (!hasTrunk) {
  console.error('\nMissing required tools. Please install them and try again.');
  process.exit(1);
}

// Check wasm32 target
try {
  const targets = execSync('rustup target list --installed', { encoding: 'utf8' });
  if (!targets.includes('wasm32-unknown-unknown')) {
    console.error('✗ wasm32-unknown-unknown target is not installed');
    console.error('  Install with: rustup target add wasm32-unknown-unknown');
    process.exit(1);
  }
  console.log('✓ wasm32-unknown-unknown target is installed');
} catch (e) {
  console.error('✗ Failed to check rustup targets');
  process.exit(1);
}

console.log('\nBuilding WASM module...');
try {
  execSync('trunk build --release', {
    cwd: ROOT,
    stdio: 'inherit'
  });
  console.log('✓ WASM build successful');
} catch (e) {
  console.error('✗ WASM build failed');
  process.exit(1);
}

console.log('\nCopying files to npm/dist/...');

// Create dist directory
fs.rmSync(DIST_DIR, { recursive: true, force: true });
fs.mkdirSync(DIST_DIR, { recursive: true });

// Copy WASM, JS, and wasm-bindgen snippet files
const files = fs.readdirSync(BUILD_OUTPUT);
const jsFiles = files.filter(f => f.endsWith('.js'));
const wasmFiles = files.filter(f => f.endsWith('.wasm'));

for (const file of jsFiles) {
  const src = path.join(BUILD_OUTPUT, file);
  const dest = path.join(DIST_DIR, file);
  fs.copyFileSync(src, dest);
  console.log(`  Copied: ${file}`);
}

const sourceWasm = wasmFiles.find(f => f === 'gridcraft-preview_bg.wasm') || wasmFiles[0];
const targetWasm = 'gridcraft-preview_bg.wasm';
if (sourceWasm) {
  fs.copyFileSync(path.join(BUILD_OUTPUT, sourceWasm), path.join(DIST_DIR, targetWasm));
  console.log(`  Copied: ${targetWasm}`);
}

function copyDirectory(source, destination) {
  fs.mkdirSync(destination, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true })) {
    const sourcePath = path.join(source, entry.name);
    const destinationPath = path.join(destination, entry.name);
    if (entry.isDirectory()) {
      copyDirectory(sourcePath, destinationPath);
    } else {
      fs.copyFileSync(sourcePath, destinationPath);
    }
  }
}

const snippetsDir = path.join(BUILD_OUTPUT, 'snippets');
if (fs.existsSync(snippetsDir)) {
  copyDirectory(snippetsDir, path.join(DIST_DIR, 'snippets'));
  console.log('  Copied: snippets/');
}

// Create main entry point that re-exports from the generated JS
const mainJsFile = jsFiles[0];
if (mainJsFile) {
  const entryPoint = `
// GridCraft Preview entry point
export * from './${mainJsFile}';
export { default } from './${mainJsFile}';
`.trim();

  fs.writeFileSync(path.join(DIST_DIR, 'index.js'), entryPoint);
  console.log('  Created: index.js');
}

// Copy TypeScript definitions
fs.copyFileSync(
  path.join(NPM_DIR, 'index.d.ts'),
  path.join(DIST_DIR, 'index.d.ts')
);
console.log('  Copied: index.d.ts');

// Copy licenses from project root
const projectRoot = path.resolve(ROOT, '../..');
for (const license of ['LICENSE-MIT', 'LICENSE-APACHE']) {
  const src = path.join(projectRoot, license);
  const dest = path.join(NPM_DIR, license);
  if (fs.existsSync(src)) {
    fs.copyFileSync(src, dest);
    console.log(`  Copied: ${license}`);
  }
}

// Get exact raw & gzipped WASM file size
if (sourceWasm) {
  const wasmPath = path.join(DIST_DIR, targetWasm);
  const rawBytes = fs.statSync(wasmPath).size;
  const gzipBytes = zlib.gzipSync(fs.readFileSync(wasmPath)).length;
  const sizeMB = (rawBytes / (1024 * 1024)).toFixed(2);
  const gzipMB = (gzipBytes / (1024 * 1024)).toFixed(2);
  console.log(`\n✓ Build complete! WASM size: ${sizeMB} MB (${rawBytes} bytes)`);
  console.log(`  Exact gzipped size: ${gzipMB} MB (${gzipBytes} bytes)`);
}

console.log('\nReady for: npm pack');
console.log('Or publish with: npm publish');
