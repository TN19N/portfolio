use crate::components::NavBar;
use crate::pages::{Home, PageNotFound};
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub enum AppRouter {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
    #[end_layout]

    #[route("/..segments")]
    PageNotFound { segments: Vec<String> },
}
