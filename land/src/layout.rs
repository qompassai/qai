use leptos::*;
use leptos_meta::*;

#[component]
pub fn RootLayout(children: Children) -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en">
            <Head>
                <Title>"Qompass AI"</Title>
                <Meta name="description" content="Your quantum compass for AI exploration."/>
                <Link rel="icon" href="/qompass.png"/>

                // Include your fonts (via stylesheet link instead of Next.js font system)
                <Link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=Geist&family=Geist+Mono&display=swap"
                />
            </Head>
            <Body class="antialiased font-geist-sans">
                // Include the children components
                {children()}
            </Body>
        </Html>
    }
}
