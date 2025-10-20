# Cloudflare Pages Deployment Guide

This guide will help you deploy the Python IDE to Cloudflare Pages.

## Prerequisites

- A Cloudflare account (free tier works fine)
- Git repository pushed to GitHub/GitLab
- Rust toolchain installed (for build)
- Node.js and npm installed (for CodeMirror bundling)

## Files Added for Cloudflare Pages

1. **`wrangler.toml`** - Cloudflare configuration file
2. **`dist/_headers`** - HTTP headers required for WASM and Web Workers
3. This deployment guide

## Quick Deploy Options

### Option 1: Connect GitHub Repository (Recommended)

1. Go to [Cloudflare Pages Dashboard](https://dash.cloudflare.com/pages)
2. Click "Create a project" → "Connect to Git"
3. Authorize Cloudflare to access your GitHub account
4. Select your repository: `jfuruness/exam_ide`
5. Configure build settings:
   - **Framework preset**: None
   - **Build command**: `npm install && npm run bundle-cm && trunk build --release`
   - **Build output directory**: `dist`
   - **Root directory**: (leave blank)
   - **Environment variables**: (none needed)
6. Click "Save and Deploy"

Cloudflare will:
- Clone your repository
- Install dependencies
- Build the project
- Deploy to a `*.pages.dev` subdomain
- Automatically rebuild on every push to master

### Option 2: Direct Upload (Manual)

If you prefer manual deployment:

1. Build the project locally:
```bash
npm install
npm run bundle-cm
trunk build --release
```

2. Install Wrangler CLI:
```bash
npm install -g wrangler
```

3. Login to Cloudflare:
```bash
wrangler login
```

4. Deploy:
```bash
wrangler pages deploy dist --project-name=python-ide
```

## Important Configuration

### CORS Headers (Already Configured)

The `dist/_headers` file sets required headers for WASM and SharedArrayBuffer:

```
/*
  Cross-Origin-Embedder-Policy: require-corp
  Cross-Origin-Opener-Policy: same-origin
  X-Content-Type-Options: nosniff
  X-Frame-Options: DENY
```

These headers are **critical** for:
- Web Workers to function
- WASM modules to load
- MicroPython to execute

**Do not remove or modify these headers.**

### Build Command Breakdown

```bash
npm install              # Install CodeMirror dependencies
npm run bundle-cm        # Bundle CodeMirror (creates static/codemirror.bundle.js)
trunk build --release    # Build Rust WASM app and create dist/
```

This produces a `dist/` directory containing:
- `index.html` - Entry point
- `exam_ide-*.wasm` - Main app (2.7MB)
- `exam_ide-*.js` - WASM glue code (38KB)
- `static/codemirror.bundle.js` - Editor (737KB)
- `static/micropython.mjs` - Python runtime (177KB)
- `static/micropython.wasm` - Python interpreter (133KB)
- `static/worker.js` - Web Worker script (3KB)
- `_headers` - HTTP headers configuration

## Deployment Verification

After deployment, test these features:

1. **Basic functionality**:
   - Visit your `*.pages.dev` URL
   - Editor should load with "Hello, World!" code
   - Click "Run" - should see output in console

2. **Keyboard shortcuts**:
   - Press `Ctrl+S` - should run code
   - Press `Shift+Enter` - should run code

3. **Persistence**:
   - Write some code
   - Refresh the page
   - Code should persist

4. **input() handling**:
   - Try running `input("test")`
   - Should see: `NotImplementedError: input() is not supported in this environment`

5. **Stop button**:
   - Run an infinite loop: `while True: pass`
   - Click "Stop" - should terminate execution

## Custom Domain (Optional)

To use a custom domain:

1. Go to your Cloudflare Pages project
2. Click "Custom domains"
3. Add your domain (e.g., `python-ide.example.com`)
4. Cloudflare will provide DNS records to configure
5. SSL certificate is automatically provisioned

## Environment-Specific Builds

### Development
```bash
trunk serve --port 8080
```

### Production
```bash
trunk build --release
```

The `--release` flag:
- Enables optimizations
- Reduces WASM size
- Removes debug symbols
- Minifies JavaScript

## Troubleshooting

### Build fails on Cloudflare

**Symptom**: Build command fails with Rust/Cargo errors

**Solution**: Check that Cloudflare has the correct Rust version. Add to build environment:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
```

Or simplify by building locally and using Option 2 (Direct Upload).

### Editor not loading

**Symptom**: Blank screen or console errors about CodeMirror

**Solution**:
- Ensure `npm run bundle-cm` ran successfully
- Check that `dist/static/codemirror.bundle.js` exists
- Verify the file is not empty (should be ~737KB)

### Python not executing

**Symptom**: "Run" button does nothing or errors

**Solution**:
- Check browser console for CORS errors
- Verify `_headers` file is in `dist/` directory
- Ensure headers are being served (check Network tab in DevTools)
- Cloudflare Pages should automatically read `_headers`

### CORS/COEP errors in browser

**Symptom**: Console shows errors like:
```
SharedArrayBuffer is not defined
Cross-Origin-Embedder-Policy blocked the load
```

**Solution**:
- Verify `dist/_headers` exists and contains correct headers
- Clear browser cache
- Check deployed site's response headers (Network tab)
- Redeploy if headers are missing

### Code not persisting

**Symptom**: Code resets after refresh

**Solution**:
- Check localStorage is enabled in browser
- Not in private/incognito mode
- Check browser console for quota errors

## Performance

Expected metrics on Cloudflare Pages:

- **First load**: ~3.8MB download (cached after first visit)
- **Subsequent loads**: Instant (served from cache)
- **Global CDN**: Fast loading worldwide
- **Free tier**: 500 builds/month, unlimited requests

## Cost

**Cloudflare Pages Free Tier**:
- ✅ Unlimited requests
- ✅ Unlimited bandwidth
- ✅ 500 builds per month
- ✅ Free SSL certificate
- ✅ Global CDN
- ✅ DDoS protection

**More than sufficient for an exam IDE.**

## Security Notes

The application is completely static:
- No backend server required
- No database connections
- No API keys needed
- All computation runs client-side

Cloudflare Pages provides:
- Automatic HTTPS
- DDoS protection
- Web Application Firewall (WAF) available
- No server to hack or maintain

## Updating the Deployment

### Automatic (GitHub connected)
Simply push to your repository:
```bash
git add .
git commit -m "Update feature"
git push
```

Cloudflare will automatically rebuild and deploy.

### Manual (Direct upload)
Rebuild and redeploy:
```bash
npm run bundle-cm
trunk build --release
wrangler pages deploy dist --project-name=python-ide
```

## Rollback

Cloudflare Pages keeps deployment history:

1. Go to your project dashboard
2. Click "Deployments"
3. Find the previous working deployment
4. Click "⋯" → "Promote to production"

## Support

If you encounter issues:

1. Check [Cloudflare Pages documentation](https://developers.cloudflare.com/pages/)
2. Review build logs in the Cloudflare dashboard
3. Test the build locally first: `trunk build --release`
4. Verify `dist/` directory structure matches expected files

## Next Steps

After deployment:
1. Share the `*.pages.dev` URL with students
2. Consider setting up a custom domain
3. Test all features in production
4. Monitor usage in Cloudflare Analytics
5. Set up preview deployments for testing changes

Your Python IDE is now live and globally distributed!
