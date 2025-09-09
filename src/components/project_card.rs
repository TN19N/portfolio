use dioxus::prelude::*;

use crate::icons::ExternalLink;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ProjectCardProps {
    link: String,
    title: String,
    technologies: Vec<&'static str>,
    description: String,
    details: String,
}

#[component]
pub fn ProjectCard(props: ProjectCardProps) -> Element {
    rsx! {
        div { class: "text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm",
            div { class: "grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6",
                div { class: "leading-none font-semibold text-card-foreground flex items-center justify-between",
                    "{props.title}"

                    a { href: "{props.link}", target: "_blank",
                        ExternalLink { class: "w-4 h-4 text-accent" }
                    }
                }
                div { class: "text-muted-foreground text-sm", "{props.description}" }
            }
            div { class: "px-6",
                div { class: "flex flex-wrap gap-2 mb-4",
                    for technologie in props.technologies {
                        span { class: "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90",
                            "{technologie}"
                        }
                    }
                }
                p { class: "text-sm text-muted-foreground", "{props.details}" }
            }
        }
    }
}
