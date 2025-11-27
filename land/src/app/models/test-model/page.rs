use leptos::*;
use leptos_router::*;

#[component]
pub fn TestModelPage() -> impl IntoView {
    view! {
        <main class="min-h-screen flex items-center justify-center bg-gray-50 p-8">
            <div class="bg-white rounded-xl shadow-lg p-6 space-y-4 max-w-xl w-full text-center border border-gray-200">
                <h1 class="text-2xl font-bold text-gray-900">"Your model is ready 🎉"</h1>
                <p class="text-gray-700">"You can find your model at:"</p>
                <pre class="bg-gray-100 p-3 rounded text-sm overflow-x-auto text-left">
                    <code>"https://qompass.ai/models/test-model"</code>
                </pre>

                // Download button
                <a
                    href="/models/test-model/model.bin"
                    download
                    class="inline-block mt-4 px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 transition"
                >
                    "Download model"
                </a>

                <A href="/" class="block text-blue-600 mt-4 hover:underline">
                    "Back to Home"
                </A>
            </div>
        </main>
    }
}
