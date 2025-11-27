use leptos::{component, view, IntoView, Scope};

mod components;
use components::ui::rose_download::RoseDownload;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <main class="p-4">
            <h1 class="text-3xl font-bold mb-4">"Welcome to Qompass!"</h1>
            <RoseDownload />
        </main>
    }
}
