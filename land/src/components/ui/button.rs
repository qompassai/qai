use leptos::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

fn button_classes(variant: ButtonVariant, size: ButtonSize) -> &'static str {
    match (variant, size) {
        (ButtonVariant::Default, ButtonSize::Default) => "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] bg-primary text-primary-foreground shadow-xs hover:bg-primary/90 h-9 px-4 py-2",
        (ButtonVariant::Destructive, ButtonSize::Default) => "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] bg-destructive text-white shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60 h-9 px-4 py-2",
        (ButtonVariant::Outline, ButtonSize::Default) => "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 h-9 px-4 py-2",
        (ButtonVariant::Secondary, ButtonSize::Default) => "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80 h-9 px-4 py-2",
        (ButtonVariant::Ghost, ButtonSize::Default) => "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50 h-9 px-4 py-2",
        (ButtonVariant::Link, ButtonSize::Default) => "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] text-primary underline-offset-4 hover:underline h-9 px-4 py-2",
        // Add other combinations as needed...
        _ => "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] h-9 px-4 py-2",
    }
}

#[component]
pub fn Button(
    #[prop(optional, default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::Default)] size: ButtonSize,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(optional)] children: Children,
) -> impl IntoView {
    view! {
        <button
            class=button_classes(variant, size)
            on:click=move |ev| {
                if let Some(cb) = &on_click {
                    cb.call(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
