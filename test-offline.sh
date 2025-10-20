#!/bin/bash

echo "==================================="
echo "Offline Python IDE - Verification"
echo "==================================="
echo ""

# Check all required files exist
echo "✓ Checking bundled files..."
required_files=(
    "dist/exam_ide-a2fff3b6c1e0c5c0_bg.wasm"
    "dist/exam_ide-a2fff3b6c1e0c5c0.js"
    "dist/index.html"
    "dist/static/codemirror.bundle.js"
    "dist/static/micropython.js"
    "dist/static/firmware.wasm"
    "dist/static/worker.js"
)

all_exist=true
for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        size=$(du -h "$file" | cut -f1)
        echo "  ✓ $file ($size)"
    else
        echo "  ✗ MISSING: $file"
        all_exist=false
    fi
done

echo ""

if [ "$all_exist" = false ]; then
    echo "❌ Some files are missing. Run 'trunk build' first."
    exit 1
fi

echo "✓ All files present"
echo ""
echo "📦 Total bundle size: $(du -sh dist | cut -f1)"
echo ""

# Check for external references
echo "✓ Checking for external dependencies..."
external_refs=$(grep -r "https\?://" dist/ --include="*.js" --include="*.html" 2>/dev/null | grep -v "localhost" | wc -l)

if [ "$external_refs" -eq 0 ]; then
    echo "  ✓ No external HTTP(S) references found"
else
    echo "  ⚠ Found $external_refs external references:"
    grep -r "https\?://" dist/ --include="*.js" --include="*.html" 2>/dev/null | grep -v "localhost"
fi

echo ""

# Check if server is running
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo "✓ Server is running on http://localhost:8080"
    echo ""
    echo "🎯 Testing Instructions:"
    echo "  1. Open http://localhost:8080 in your browser"
    echo "  2. Write Python code in the editor"
    echo "  3. Click 'Run' button"
    echo "  4. Verify output appears in console"
    echo "  5. Click 'Stop' button to test kill"
    echo "  6. Reload page to test persistence"
    echo "  7. Open DevTools > Network > Enable 'Offline' mode"
    echo "  8. Refresh - should work from cache"
else
    echo "⚠ Server not running. Start with:"
    echo "  cd dist && python3 -m http.server 8080"
fi

echo ""
echo "✅ Verification complete!"
echo ""
echo "For deployment instructions, see DEPLOYMENT.md"
