use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use flate2::write::{DeflateEncoder, DeflateDecoder};
use flate2::Compression;
use std::io::Write;
use wasm_bindgen::JsValue;
use web_sys::window;

/// Compress and encode code for URL
pub fn encode_for_url(code: &str) -> Result<String, String> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(code.as_bytes())
        .map_err(|e| format!("Compression failed: {}", e))?;

    let compressed = encoder.finish()
        .map_err(|e| format!("Compression finish failed: {}", e))?;

    Ok(URL_SAFE_NO_PAD.encode(&compressed))
}

/// Decode and decompress code from URL
pub fn decode_from_url(encoded: &str) -> Result<String, String> {
    let compressed = URL_SAFE_NO_PAD.decode(encoded)
        .map_err(|e| format!("Base64 decode failed: {}", e))?;

    let mut decoder = DeflateDecoder::new(Vec::new());
    decoder.write_all(&compressed)
        .map_err(|e| format!("Decompression failed: {}", e))?;

    let decompressed = decoder.finish()
        .map_err(|e| format!("Decompression finish failed: {}", e))?;

    String::from_utf8(decompressed)
        .map_err(|e| format!("UTF-8 decode failed: {}", e))
}

/// Get code from URL hash
pub fn get_code_from_hash() -> Option<String> {
    let window = window()?;
    let location = window.location();
    let hash = location.hash().ok()?;

    if hash.is_empty() || hash == "#" {
        return None;
    }

    // Remove leading #
    let encoded = hash.trim_start_matches('#');

    decode_from_url(encoded).ok()
}

/// Set code in URL hash
pub fn set_code_in_hash(code: &str) -> Result<(), String> {
    let window = window().ok_or("No window")?;
    let location = window.location();

    let encoded = encode_for_url(code)?;
    let hash = format!("#{}", encoded);

    location.set_hash(&hash)
        .map_err(|e| format!("Failed to set hash: {:?}", e))?;

    Ok(())
}
