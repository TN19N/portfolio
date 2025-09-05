use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// A layout component that displays a site-wide navbar and renders the matched route inside an outlet.
///
/// The Navbar injects its stylesheet, renders a "Home" link to `Route::Home`, and provides an `Outlet::<Route>`
/// where the current route's child (e.g., `Home` or `Blog`) will be rendered.
///
/// # Examples
///
/// ```no_run
/// use dioxus::prelude::*;
/// use crate::views::navbar::Navbar;
///
/// // Use the Navbar in your app's root component
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         Navbar {}
///     })
/// }
/// ```
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
