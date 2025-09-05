use crate::components::Hero;
use dioxus::prelude::*;

/// Home page component for the `[Route::Home]` route.
///
/// Renders the `Hero` component as the page's content.
///
/// # Examples
///
/// ```ignore
/// // inside a Dioxus render context:
/// rsx! {
///     Home {}
/// }
/// ```
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}
