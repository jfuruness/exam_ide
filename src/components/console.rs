use leptos::*;

#[component]
pub fn Console(
    #[prop(into)] output: Signal<String>,
    #[prop(into)] is_running: Signal<bool>,
) -> impl IntoView {
    let console_ref = create_node_ref::<html::Pre>();

    // Auto-scroll to bottom when output changes
    create_effect(move |_| {
        output.track();
        if let Some(element) = console_ref.get() {
            element.set_scroll_top(element.scroll_height());
        }
    });

    view! {
        <div style="height: 100%; display: flex; flex-direction: column; background: #1e1e1e; color: #d4d4d4;">
            <div style="padding: 10px; background: #2d2d30; border-bottom: 1px solid #3e3e42; font-weight: bold; color: #cccccc;">
                "Output"
                {move || {
                    if is_running.get() {
                        view! { <span style="margin-left: 10px; color: #4ec9b0;">" (Running...)"</span> }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }
                }}
            </div>
            <pre
                node_ref=console_ref
                style="flex: 1; margin: 0; padding: 15px; overflow-y: auto; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace; font-size: 13px; line-height: 1.5; white-space: pre-wrap; word-wrap: break-word;"
            >
                {move || output.get()}
            </pre>
        </div>
    }
}
