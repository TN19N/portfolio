use crate::components::{Contact, Experiences, Hero, Projects};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}

        Projects {}

        Experiences {}

        Contact {}

        // Footer
        footer { class: "py-8 border-t border-border bg-card/50",
            div { class: "container mx-auto px-4 text-center",
                p { class: "text-muted-foreground",
                    "© 2025 TN19N. Built with "
                    em {
                        a {
                            href: "https://dioxuslabs.com/",
                            color: "blue",
                            target: "__blank",
                            "Dioxus"
                        }
                    }
                    "."
                }
            }
        }
    }
}
