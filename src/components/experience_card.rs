use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ExperienceYearsDuration {
    pub from: u32,
    pub to: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ExperienceCardProps {
    role: String,
    company: String,
    duration: ExperienceYearsDuration,
    tasks: Vec<&'static str>,
}

#[component]
pub fn ExperienceCard(props: ExperienceCardProps) -> Element {
    rsx! {
        div { class: "text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm bg-card border-border",
            div { class: "grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6",
                div { class: "flex justify-between items-start",
                    div {
                        div { class: "leading-none font-semibold text-card-foreground",
                            "{props.role}"
                        }
                        div { class: "text-sm text-accent font-medium", "{props.company}" }
                    }
                    span { class: "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden [a&]:hover:bg-accent [a&]:hover:text-accent-foreground border-primary text-primary",
                        "{props.duration.from} - "
                        if let Some(year) = props.duration.to {
                            "{year}"
                        } else {
                            "Present"
                        }
                    }
                }
            }
            div { class: "px-6",
                ul { class: "list-disc list-inside space-y-2 text-muted-foreground",
                    for task in props.tasks {
                        li { "{task}" }
                    }
                }
            }
        }
    }
}
