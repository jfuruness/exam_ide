use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::python_ide::PythonIde;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                // Redirect root to /python
                <Route path="/" view=move || {
                    view! {
                        <Redirect path="/python"/>
                    }
                }/>

                // Python IDE route
                <Route path="/python" view=PythonIde/>

                // Catch-all redirect to /python
                <Route path="/*any" view=move || {
                    view! {
                        <Redirect path="/python"/>
                    }
                }/>
            </Routes>
        </Router>
    }
}
