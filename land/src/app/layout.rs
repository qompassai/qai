use leptos::*;
use leptos_meta::*;

#[component]
pub fn RootLayout(children: Children) -> impl IntoView {
    // Provide meta context for head elements
    provide_meta_context();

    view! {
        <Html lang="en">
            <Head>
                <Title>"Qompass AI"</Title>
                <Meta name="description" content="Your quantum compass for AI exploration."/>
                <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Geist&family=Geist+Mono&display=swap"/>
                <Link rel="icon" href="/qompass.png"/>
            </Head>
            <Body class="antialiased font-sans">
                {children()}
            </Body>
        </Html>
    }
}
