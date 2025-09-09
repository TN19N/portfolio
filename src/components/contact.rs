use crate::icons::{Linkedin, Mail};
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        section { id: "contact", class: "py-20 bg-muted/30",
            div { class: "container mx-auto px-4",
                h3 { class: "text-3xl font-bold text-center mb-12 text-card-foreground",
                    "Get In Touch"
                }
                div { class: "flex justify-center gap-6 mt-8",
                    for icon in [rsx! {
                        a { href: "https://www.linkedin.com/in/mustapha-annouaoui", target: "_blank",
                            Linkedin { class: "w-5 h-5" }
                        }
                    }, rsx! {
                        a { href: "mailto:mostafaanawawi@gmail.com", target: "_blank",
                            Mail { class: "w-5 h-5" }
                        }
                    }]
                    {
                        button { class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive border shadow-xs dark:bg-input/30 dark:border-input dark:hover:bg-input/50 size-9 border-primary text-primary hover:bg-primary hover:text-primary-foreground bg-transparent",
                            {icon}
                        }
                    }
                }
            }
        }
    }
}
