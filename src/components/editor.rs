use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "CodeMirrorSetup"])]
    fn createEditor(
        parent: &HtmlElement,
        initial_content: &str,
        on_change: &Closure<dyn FnMut(String)>,
    ) -> JsValue;
}

#[component]
pub fn Editor(
    #[prop(into)] code: RwSignal<String>,
) -> impl IntoView {
    let editor_ref = create_node_ref::<html::Div>();
    let editor_instance = store_value(None::<JsValue>);

    create_effect(move |_| {
        if let Some(element) = editor_ref.get() {
            let code_value = code.get_untracked();

            let on_change = Closure::wrap(Box::new(move |new_code: String| {
                code.set(new_code);
            }) as Box<dyn FnMut(String)>);

            let instance = createEditor(&element, &code_value, &on_change);
            editor_instance.set_value(Some(instance));

            // Keep the closure alive
            on_change.forget();
        }
    });

    view! {
        <div
            node_ref=editor_ref
            style="height: 100%; width: 100%; overflow: hidden;"
        />
    }
}
