use leptos::*;

#[component]
pub fn Card(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm";
    view! {
        <div class=format!("{} {}", base, class)>{children()}</div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6";
    view! {
        <div class=format!("{} {}", base, class)>{children()}</div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "leading-none font-semibold";
    view! {
        <div class=format!("{} {}", base, class)>{children()}</div>
    }
}

#[component]
pub fn CardDescription(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "text-muted-foreground text-sm";
    view! {
        <div class=format!("{} {}", base, class)>{children()}</div>
    }
}

#[component]
pub fn CardAction(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "col-start-2 row-span-2 row-start-1 self-start justify-self-end";
    view! {
        <div class=format!("{} {}", base, class)>{children()}</div>
    }
}

#[component]
pub fn CardContent(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "px-6";
    view! {
        <div class=format!("{} {}", base, class)>{children()}</div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(optional, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex items-center px-6 [.border-t]:pt-6";
    view! {
        <div class=format!("{} {}", base, class)>{children()}</div>
    }
}
