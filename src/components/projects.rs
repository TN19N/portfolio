use crate::components::project_card::ProjectCard;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { id: "projects", class: "py-20",
            div { class: "container mx-auto px-4",
                h3 { class: "text-3xl font-bold text-center mb-12 text-card-foreground",
                    "Featured Projects"
                }
                div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-6",
                    ProjectCard {
                        link: "https://github.com/tn19n/subscriptions",
                        title: "Subscriptions",
                        description: "The goal of this project is to build a robust and production-ready subscription service while exploring the Rust ecosystem.",
                        details: "This project is a learning exercise following the principles and best practices outlined in the `Zero to Production in Rust` book by Luca Palmieri.",
                        technologies: vec!["Rust", "Axum", "Tokio", "SurrealDB", "CI/CD", "Moonrepo", "Testing"],
                    }
                    ProjectCard {
                        link: "https://github.com/TN19N/ft_transcendence",
                        title: "Ft_Transcendence",
                        description: "This project is about creating a Ping-Pong website where users will play Ping-Pong game in real-time with others, with a nice user interface, a chat, and real-time notification and messaging.",
                        details: "built core APIs, JWT-based authentication, real-time sockets, and CI/CD with GitHub Actions.",
                        technologies: vec![
                            "NestJs",
                            "Socket.IO",
                            "Docker",
                            "CI/CD",
                            "Postgres",
                            "Prisma",
                            "Jwt",
                            "OAuth",
                            "Jest",
                        ],
                    }
                    ProjectCard {
                        link: "https://github.com/tn19n/WebServ",
                        title: "WebServ",
                        description: "This project is about writing My own HTTP server (HTTP/1.1). and be able to test it with an actual browser.",
                        details: "Designed non-blocking I/O with multiplexing to handle concurrent client connections.",
                        technologies: vec!["C/C++", "OOP", "Http", "I/O", "Multiplexing"],
                    }
                }
            }
        }
    }
}
