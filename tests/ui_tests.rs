//! Integration tests for the Navbar component.
//! Test stack: Rust built-in test harness + dioxus-ssr for deterministic SSR.

use dioxus::prelude::*;
use dioxus_ssr as ssr;

/// Helper: render the Navbar component to HTML via SSR.
fn render_navbar_html() -> String {
    let mut vdom = VirtualDom::new(portfolio::Navbar);
    // Rebuild the VirtualDom before rendering (API stable across Dioxus 0.5/0.6).
    vdom.rebuild_in_place();
    ssr::render(&vdom)
}

#[test]
fn navbar_renders_home_link_text() {
    let html = render_navbar_html();
    assert!(
        html.contains("Home"),
        "Expected rendered HTML to contain the 'Home' link text. Got: {html}"
    );
}

#[test]
fn navbar_has_navbar_id_attribute() {
    let html = render_navbar_html();
    assert!(
        html.contains("id=\"navbar\"") || html.contains("id='navbar'"),
        "Expected a root element with id=\"navbar\". Got: {html}"
    );
}

#[test]
fn navbar_includes_stylesheet_link_for_navbar_css() {
    let html = render_navbar_html();
    assert!(
        html.contains("rel=\"stylesheet\"") || html.contains("rel='stylesheet'"),
        "Expected a stylesheet <link> tag. Got: {html}"
    );
    assert!(
        html.contains("navbar.css"),
        "Expected the stylesheet href to reference navbar.css. Got: {html}"
    );
}

#[test]
fn navbar_ssr_output_is_stable_between_renders() {
    let first = render_navbar_html();
    let second = render_navbar_html();
    assert_eq!(
        first, second,
        "SSR output should be stable for this static component."
    );
}

#[test]
fn navbar_renders_without_router_context() {
    // The component contains an Outlet::<Route> {}. Ensure it does not panic when rendered
    // without an explicit Router context (should simply render the layout).
    let result = std::panic::catch_unwind(|| {
        let _ = render_navbar_html();
    });
    assert!(result.is_ok(), "Rendering Navbar should not panic without Router context.");
}