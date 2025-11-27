use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    // State management using signals instead of useState
    let (prompt, set_prompt) = create_signal(String::new());
    let (response, set_response) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);

    // Event handler for form submission
    let handle_submit = move |_| {
        set_loading.update(|loading| *loading = true);

        // Clone values for the async block
        let prompt_value = prompt.get();

        spawn_local(async move {
            match fetch_response(&prompt_value).await {
                Ok(result) => set_response.set(result),
                Err(_) => set_response.set("Something went wrong.".to_string()),
            }
            set_loading.set(false);
        });
    };

    view! {
        <main class="min-h-screen bg-gradient-to-b from-sky-50 via-slate-50 to-indigo-100 text-slate-900 flex flex-col">
            // Navbar
            <nav class="w-full px-6 py-4 flex justify-between items-center bg-slate-950 text-white shadow">
                <h1 class="text-xl font-semibold tracking-wide">"⚓ Qompass AI"</h1>
                <div class="space-x-6 text-sm">
                    <A href="/" class="hover:underline">"Nautilus"</A>
                    <A href="/equator" class="hover:underline">"Equator"</A>
                    <A href="/sojourn" class="hover:underline">"Sojourn"</A>
                    <A href="/waverunner" class="hover:underline">"WaveRunner"</A>
                </div>
            </nav>

            // Hero Section
            <section class="flex-1 flex flex-col items-center justify-center px-4 py-12 text-center">
                <h2 class="text-4xl sm:text-5xl font-extrabold text-blue-900 tracking-tight">
                    "Navigating Fairness in Medical AI"
                </h2>
                <p class="mt-4 max-w-2xl text-lg text-slate-700">
                    "Qompass AI helps guide the future of medical education with ethical, transparent, and human-centered AI tools."
                </p>

                <div class="mt-6 flex flex-col sm:flex-row gap-4">
                    <button
                        class="bg-blue-800 hover:bg-blue-700 text-white px-6 py-2 rounded-lg shadow text-base"
                        on:click=handle_submit
                        disabled=loading
                    >
                        {move || if loading.get() { "Thinking..." } else { "Try Demo" }}
                    </button>

                    <a
                        href="/rose"
                        download="rose-linux"
                        class="inline-flex items-center justify-center bg-white text-blue-800 border border-blue-700 px-6 py-2 rounded-lg shadow hover:bg-blue-50 transition text-base"
                    >
                        "Download Rose ⬇"
                    </a>
                </div>

                // Prompt
                <div class="mt-8 w-full max-w-xl p-6 bg-white shadow-xl space-y-4 rounded-lg">
                    <textarea
                        placeholder="Ask about medical AI, fairness, or admissions bias..."
                        prop:value=move || prompt.get()
                        on:input=move |ev| {
                            set_prompt.set(event_target_value(&ev));
                        }
                        class="w-full p-2 border border-gray-300 rounded"
                    />
                    <button
                        on:click=handle_submit
                        disabled=loading
                        class="w-full bg-blue-700 text-white py-2 rounded"
                    >
                        {move || if loading.get() { "Analyzing..." } else { "Submit" }}
                    </button>
                    {move || {
                        let response_text = response.get();
                        if !response_text.is_empty() {
                            view! {
                                <div class="p-4 bg-blue-50 border border-blue-200 shadow-inner whitespace-pre-line rounded-lg">
                                    {response_text}
                                </div>
                            }
                        } else {
                            view! { <></> }
                        }
                    }}
                </div>

                // Social Icons
                <div class="mt-10 flex flex-wrap gap-6 items-center justify-center">
                    <a href="https://github.com/qompassai" target="_blank" rel="noreferrer" title="GitHub">
                        <img src="/icons/github.svg" alt="GitHub" class="h-8 hover:scale-110 transition-transform" />
                    </a>
                    <a href="https://huggingface.co/qompassai" target="_blank" rel="noreferrer" title="Hugging Face">
                        <img src="/icons/huggingface.svg" alt="Hugging Face" class="h-8 hover:scale-110 transition-transform" />
                    </a>
                    <a href="https://orcid.org/0000-0002-0302-4812" target="_blank" rel="noreferrer" title="ORCID">
                        <img src="/icons/orcid.svg" alt="ORCID" class="h-8 hover:scale-110 transition-transform" />
                    </a>
                    <a href="https://www.researchgate.net/profile/Matt-Porter-7" target="_blank" rel="noreferrer" title="ResearchGate">
                        <img src="/icons/researchgate.svg" alt="ResearchGate" class="h-8 hover:scale-110 transition-transform" />
                    </a>
                    <a href="https://www.linkedin.com/company/95058568/" target="_blank" rel="noreferrer" title="LinkedIn">
                        <img src="/icons/linkedin.svg" alt="LinkedIn" class="h-8 hover:scale-110 transition-transform" />
                    </a>
                    <a href="https://www.youtube.com/@qompassai" target="_blank" rel="noreferrer" title="YouTube">
                        <img src="/icons/youtube.svg" alt="YouTube" class="h-8 hover:scale-110 transition-transform" />
                    </a>
                </div>
            </section>

            // Footer
            <footer class="w-full py-6 text-center text-sm text-slate-600 bg-slate-50 border-t">
                <span>"© " {js_sys::Date::new_0().get_full_year()} " Qompass AI · Built for fairness in healthcare"</span>
            </footer>
        </main>
    }
}

// API function (server function or client-side fetch)
#[server(FetchResponse, "/api/chat")]
pub async fn fetch_response(prompt: &str) -> Result<String, ServerFnError> {
    // Server-side implementation
    Ok("This is a mock response for your prompt about medical AI.".to_string())
}

// For client-side only implementation:
async fn fetch_response(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create request
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(&format!(
        "{{\"prompt\":\"{}\"}}",
        prompt
    ))));

    let request = web_sys::Request::new_with_str_and_init("/api/chat", &opts)?;

    request.headers().set("Content-Type", "application/json")?;

    // Fetch
    let window = web_sys::window().unwrap();
    let resp_value =
        wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;

    // Parse response
    let json = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
    let response = js_sys::Reflect::get(&json, &JsValue::from_str("response"))?;

    Ok(response
        .as_string()
        .unwrap_or_else(|| "Error parsing response".to_string()))
}
