use leptos::*;
use leptos_meta::*;

use crate::components::editor::Editor;
use crate::components::console::Console;
use crate::python_runner::{PythonRunner, WorkerResponse};
use crate::storage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Load initial code from localStorage or use default
    let initial_code = storage::get_default_code();
    let code = create_rw_signal(initial_code);
    let output = create_rw_signal(String::new());
    let is_running = create_rw_signal(false);
    let runner = store_value(None::<PythonRunner>);

    // Save code to localStorage whenever it changes
    create_effect(move |_| {
        let current_code = code.get();
        storage::save_code(&current_code);
    });

    let run_code = move |_| {
        if is_running.get() {
            return;
        }

        is_running.set(true);
        output.set(String::new());

        let current_code = code.get_untracked();

        // Create a new worker for each execution
        let new_runner = PythonRunner::new(move |response| {
            match response {
                WorkerResponse::Output { text } => {
                    output.update(|o| o.push_str(&text));
                }
                WorkerResponse::Error { text } => {
                    output.update(|o| {
                        o.push_str(&format!("Error: {}\n", text));
                    });
                    is_running.set(false);
                }
                WorkerResponse::Done => {
                    is_running.set(false);
                }
            }
        });

        match new_runner {
            Ok(r) => {
                if let Err(e) = r.run_code(current_code) {
                    output.set(format!("Failed to start execution: {:?}\n", e));
                    is_running.set(false);
                } else {
                    runner.set_value(Some(r));
                }
            }
            Err(e) => {
                output.set(format!("Failed to create runner: {:?}\n", e));
                is_running.set(false);
            }
        }
    };

    let stop_code = move |_| {
        runner.with_value(|r| {
            if let Some(runner_instance) = r {
                let _ = runner_instance.stop();
            }
        });
        output.update(|o| o.push_str("\n--- Execution stopped by user ---\n"));
        is_running.set(false);
    };

    view! {
        <Html lang="en"/>
        <Title text="Python IDE"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <div style="height: 100vh; display: flex; flex-direction: column; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;">
            // Header
            <div style="background: #2d2d30; color: white; padding: 12px 20px; display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid #3e3e42;">
                <h1 style="margin: 0; font-size: 18px; font-weight: 600;">"Python IDE"</h1>
                <div style="display: flex; gap: 10px;">
                    <button
                        on:click=run_code
                        disabled=move || is_running.get()
                        style="padding: 8px 16px; background: #0e639c; color: white; border: none; border-radius: 3px; cursor: pointer; font-size: 14px; font-weight: 500; transition: background 0.2s;"
                        style:opacity=move || if is_running.get() { "0.5" } else { "1.0" }
                        style:cursor=move || if is_running.get() { "not-allowed" } else { "pointer" }
                    >
                        {move || if is_running.get() { "Running..." } else { "▶ Run" }}
                    </button>
                    <button
                        on:click=stop_code
                        disabled=move || !is_running.get()
                        style="padding: 8px 16px; background: #c5383c; color: white; border: none; border-radius: 3px; cursor: pointer; font-size: 14px; font-weight: 500; transition: background 0.2s;"
                        style:opacity=move || if is_running.get() { "1.0" } else { "0.5" }
                        style:cursor=move || if is_running.get() { "pointer" } else { "not-allowed" }
                    >
                        "⬛ Stop"
                    </button>
                </div>
            </div>

            // Main content area
            <div style="flex: 1; display: flex; overflow: hidden;">
                // Editor pane
                <div style="flex: 1; display: flex; flex-direction: column; border-right: 1px solid #3e3e42;">
                    <div style="padding: 10px; background: #f7f7f7; border-bottom: 1px solid #ddd; font-weight: bold; color: #333;">
                        "editor.py"
                    </div>
                    <div style="flex: 1; overflow: hidden;">
                        <Editor code=code/>
                    </div>
                </div>

                // Console pane
                <div style="flex: 1; display: flex; flex-direction: column;">
                    <Console output=output.read_only() is_running=is_running.read_only()/>
                </div>
            </div>

            // Footer
            <div style="background: #007acc; color: white; padding: 6px 20px; font-size: 12px; display: flex; align-items: center; justify-content: space-between;">
                <span>"Ready"</span>
                <span>"MicroPython WASM"</span>
            </div>
        </div>
    }
}
