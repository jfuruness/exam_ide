use wasm_bindgen::JsValue;
use web_sys::window;

const STORAGE_KEY: &str = "python_code";

pub fn save_code(code: &str) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.set_item(STORAGE_KEY, code);
    }
}

pub fn load_code() -> Option<String> {
    window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|storage| storage.get_item(STORAGE_KEY).ok())
        .flatten()
}

pub fn get_default_code() -> String {
    load_code().unwrap_or_else(|| {
        r#"# Welcome to Python IDE!
# Write your Python code here and click Run

print("Hello, World!")

for i in range(5):
    print(f"Count: {i}")
"#
        .to_string()
    })
}
