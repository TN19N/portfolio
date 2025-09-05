use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

/// Renders the hero section: a header SVG and a set of static links to Dioxus resources.
///
/// The component is static and returns an Element containing an image (HEADER_SVG) with id
/// "header" and a "links" container with six anchor elements pointing to documentation, community,
/// tooling, and extension pages.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// #[component]
/// fn App(cx: Scope) -> Element {
///     cx.render(rsx!(Hero {}))
/// }
/// ```
#[component]
pub fn Hero() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                // The RSX macro also supports text nodes surrounded by quotes
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
