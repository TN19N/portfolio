use crate::router::AppRouter;
use dioxus::prelude::*;

static TAILWIND: Asset = asset!("./assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND }
        div { class: "min-h-screen bg-background", Router::<AppRouter> {} }
    }
}
