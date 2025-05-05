use leptos::*;
use leptos_meta::*;

#[component]
pub fn ArchivePage() -> impl IntoView {
    // Set page metadata
    provide_meta_context();

    view! {
        <Title text="Qompass Archive - Model Collection" />
        <Meta name="description" content="Browse our collection of AI models for local deployment" />

        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-6">"Qompass Archive"</h1>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                // Example model card - repeat for each model
                <div class="border rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow">
                    <h2 class="text-xl font-semibold mb-2">"Model Name"</h2>
                    <p class="text-gray-600 mb-4">"Brief description of the model and its capabilities."</p>
                    <div class="flex justify-between items-center">
                        <span class="text-sm bg-blue-100 text-blue-800 px-2 py-1 rounded">"7B Parameters"</span>
                        <a href="#" class="text-blue-600 hover:underline">"Details →"</a>
                    </div>
                </div>

                // You can add more model cards here
            </div>
        </div>
    }
}
