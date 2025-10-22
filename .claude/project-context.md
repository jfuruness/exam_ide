# Project Context

## Architecture Requirements

### Pure Client-Side SPA (Single Page Application)

**Critical Requirement**: This application MUST be purely client-side with NO network calls.

- **No backend server** - Everything runs in the browser
- **No API calls** - All functionality is local (WASM + localStorage)
- **SPA routing** - Using `leptos_router` for client-side navigation
- **Why**: This is a privacy-focused IDE that runs entirely offline after initial load

**Technologies**:
- Leptos (Rust → WASM) for UI framework
- MicroPython WASM for Python execution (runs in Web Worker)
- CodeMirror for code editing
- localStorage for code persistence
- URL hash for code sharing (compressed with flate2/base64)

## Security Decisions

### SRI (Subresource Integrity) - Disabled for Self-Hosted Assets

**Decision**: Disabled SRI checks using `data-no-sri` attribute in `index.html`

**Why this is NOT a security vulnerability:**

1. **Controlled deployment pipeline**
   - We control the entire pipeline: GitHub → Cloudflare Pages
   - Not loading resources from untrusted third-party CDNs
   - If deployment is compromised, attacker could change SRI hashes anyway

2. **Same-origin resources**
   - JS/WASM files are served from same domain (`examide.com`)
   - Not loading from external CDNs where SRI would be critical

3. **Existing security measures**
   - HTTPS encryption protects against MITM attacks
   - CORS headers configured (`Cross-Origin-Embedder-Policy`, `Cross-Origin-Opener-Policy`)
   - Trusted deployment pipeline (GitHub Actions → Cloudflare Pages)

4. **SRI primary use case**
   - Protects against compromised **third-party** CDNs
   - Example: Loading jQuery from `cdnjs.cloudflare.com`
   - Not applicable when serving your own built assets

**Trade-off**: Allows Cloudflare Pages to optimize (minify, compress) our files without integrity hash mismatches, improving performance.

**When SRI matters**: Loading third-party scripts from external CDNs (not our use case).
