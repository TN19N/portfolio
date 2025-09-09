use crate::components::experience_card::{ExperienceCard, ExperienceYearsDuration};
use dioxus::prelude::*;

#[component]
pub fn Experiences() -> Element {
    rsx! {
        section { id: "experiences", class: "py-20",
            div { class: "container mx-auto px-4",
                h3 { class: "text-3xl font-bold text-center mb-12 text-card-foreground",
                    "Experience"
                }
                div { class: "max-w-3xl mx-auto space-y-8",
                    ExperienceCard {
                        role: "Backend Developer",
                        company: "ORA Technologies",
                        duration: ExperienceYearsDuration {
                            from: 2024,
                            to: None,
                        },
                        tasks: vec![
                            "Designed and implemented secure, high-performance RESTful APIs and server-side logic using NestJS and PostgreSQL.",
                            "Developed and maintained backend architecture for a scalable payment application, incorporating Kafka for messaging.",
                            "Built and optimized the backend for the administrative panel, improving management efficiency.",
                        ],
                    }
                }
            }
        }
    }
}
