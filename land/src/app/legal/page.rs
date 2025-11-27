use leptos::*;
use leptos_meta::*;

#[component]
pub fn LegalPage() -> impl IntoView {
    // Set page metadata
    provide_meta_context();
    
    view! {
        <Title text="Legal Information - Qompass AI" />
        <Meta name="description" content="License and legal information for Qompass AI products and models" />
        
        <div class="container mx-auto px-4 py-8 max-w-4xl">
            <h1 class="text-3xl font-bold mb-6">"Legal Information"</h1>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"Licensing"</h2>
                <p class="mb-4">
                    "Qompass AI software is dual-licensed under:"
                </p>

                <div class="bg-gray-50 p-4 rounded-lg mb-6">
                    <h3 class="text-lg font-medium mb-2">"GNU Affero General Public License (AGPL) v3.0"</h3>
                    <p class="text-gray-700">
                        "For non-commercial use. The AGPL-3.0 license applies to the source code and ensures that modifications
                        to the software remain open source when used in a networked context."
                    </p>
                    <a href="/licenses/agpl-3.0.txt" class="text-blue-600 hover:underline mt-2 inline-block">
                        "Read full AGPL-3.0 license text"
                    </a>
                </div>

                <div class="bg-gray-50 p-4 rounded-lg">

