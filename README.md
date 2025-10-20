# Python IDE - Offline Student Exam Environment

A fully offline Python IDE built with Leptos (Rust/WASM) for student exams. Features:
- **Offline-first**: All assets bundled, no CDN or network calls
- **MicroPython WASM**: Python interpreter runs entirely in browser
- **CodeMirror editor**: Syntax highlighting and indentation only (no autocomplete)
- **Safe execution**: Web Worker isolation with 30-second timeout
- **Persistent storage**: Code saved to localStorage across sessions
- **Kill switch**: Students can stop infinite loops

## Why This Stack?

* **Leptos**: Compiles to WASM binary - students can't modify execution logic
* **MicroPython**: Lightweight, production-ready Python in WASM
* **CodeMirror**: Lightweight editor with precise control over features
* **No network**: Guaranteed offline operation for exam integrity

## Prerequisites

- Rust (latest stable)
- Node.js and npm
- wasm-pack or trunk

## Setup

### Installing MicroPython WASM

The MicroPython WASM files are already included in `static/`, but if you need to update them:

```bash
# Download the latest official MicroPython WASM from PyScript
curl -o static/micropython.mjs https://unpkg.com/@micropython/micropython-webassembly-pyscript@latest/micropython.mjs
curl -o static/micropython.wasm https://unpkg.com/@micropython/micropython-webassembly-pyscript@latest/micropython.wasm
```

**Note**: We use the official browser-compatible MicroPython from the PyScript team (`@micropython/micropython-webassembly-pyscript`), NOT the Node.js version from npm's `micropython` package.

**Why this version?**
- **Small**: ~500KB total (103KB JS + 425KB WASM)
- **Fast**: Millisecond startup time
- **Official**: Built by PyScript team from MicroPython source
- **Browser-native**: Uses ES6 modules, works in Web Workers

**Alternatives considered:**
- **Pyodide**: 6-8MB, slow startup, overkill for basic Python
- **Build from source**: Hours of compilation, produces same result
- **npm micropython**: Node.js only, uses `require()`, won't work in browsers

## Building

### Option 1: Using Trunk (recommended)

```bash
cargo install --locked trunk
npm install
trunk serve --open
```

### Option 2: Using build script

```bash
chmod +x build.sh
./build.sh
cd dist && python3 -m http.server 8080
```

### Option 3: Manual build

```bash
npm install
npm run bundle-cm
wasm-pack build --target web --out-dir dist/pkg --no-typescript --release
# Copy static files and index.html to dist/
```

## Development

```bash
trunk serve
```

## Architecture

```
├── src/
│   ├── lib.rs                 # Entry point
│   ├── components/
│   │   ├── app.rs             # Main application
│   │   ├── editor.rs          # CodeMirror wrapper
│   │   └── console.rs         # Output console
│   ├── python_runner.rs       # Web Worker communication
│   └── storage.rs             # localStorage persistence
├── static/
│   ├── worker.js              # MicroPython Web Worker
│   ├── micropython.js         # MicroPython runtime
│   ├── firmware.wasm          # MicroPython WASM binary
│   └── codemirror.bundle.js   # Bundled CodeMirror
└── index.html                 # HTML template

```

## Security Features

1. **Binary execution**: Leptos compiles to WASM - no source inspection
2. **No network**: All dependencies bundled locally
3. **Timeout protection**: 30-second execution limit
4. **Worker isolation**: Python runs in separate thread
5. **Manual stop**: Kill button for runaway code

## Deployment

The built application (`dist/` directory) is completely self-contained and can be:
- Served from any static web server
- Run offline after first load
- Deployed to USB drives for air-gapped environments
