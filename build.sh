#!/bin/bash
set -e

# Build CodeMirror bundle
echo "Building CodeMirror..."
npm install
npm run bundle-cm

# Build WASM
echo "Building WASM..."
wasm-pack build --target web --out-dir dist/pkg --no-typescript --release

# Create dist directory structure
echo "Creating distribution..."
mkdir -p dist
cp index.html dist/
cp -r static dist/

# Create a simple index loader that imports the WASM
cat > dist/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Python IDE</title>
    <script defer src="codemirror.bundle.js"></script>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            height: 100vh;
            overflow: hidden;
        }
    </style>
</head>
<body>
    <script type="module">
        import init from './pkg/exam_ide.js';
        await init();
    </script>
</body>
</html>
EOF

echo "Build complete! Output in dist/"
echo "To serve locally: cd dist && python3 -m http.server 8080"
