use crate::icons::{Download, GitHub, Mail};
use dioxus::prelude::*;

const CV: Asset = asset!("./assets/cv.pdf");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "about",
            class: "py-20 bg-gradient-to-br from-background to-muted",
            div { class: "container mx-auto px-4 text-center",
                h2 { class: "text-5xl font-bold text-primary mb-4", "Backend Developer" }
                p { class: "text-xl text-accent font-medium mb-6",
                    "Building scalable systems & robust APIs"
                }
                p { class: "text-lg text-muted-foreground max-w-2xl mx-auto mb-8 leading-relaxed",
                    "Passionate about creating efficient, maintainable backend solutions. Experienced in microservices architecture, database optimization, and cloud infrastructure with a focus on performance and reliability."
                }
                div { class: "flex items-center justify-center gap-4",
                    a { href: "#contact",
                        button { class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive text-primary-foreground shadow-xs h-9 px-4 py-2 has-[>svg]:px-3 bg-primary hover:bg-primary/90",
                            Mail { class: "w-4 h-4 mr-2" }
                            "Get In touch"
                        }
                    }
                    a { href: "https://github.com/TN19N", target: "_blank",
                        button { class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive border shadow-xs dark:bg-input/30 dark:border-input dark:hover:bg-input/50 h-9 px-4 py-2 has-[>svg]:px-3 border-primary text-primary hover:bg-primary hover:text-primary-foreground bg-transparent",
                            GitHub { class: "w-4 h-4 mr-2" }
                            "View Github"
                        }
                    }

                    a { href: CV, download: "TN19N_CV.pdf",
                        button { class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive border shadow-xs dark:bg-input/30 dark:border-input dark:hover:bg-input/50 h-9 px-4 py-2 has-[>svg]:px-3 border-primary text-primary hover:bg-primary hover:text-primary-foreground bg-transparent",
                            Download { class: "w-4 h-4 mr-2" }
                            "Download CV"
                        }
                    }
                }
            }
        }
    }
}
