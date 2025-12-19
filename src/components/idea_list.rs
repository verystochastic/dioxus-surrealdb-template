use crate::server_functions::get_all_ideas_server;
use dioxus::prelude::*;

const IDEA_LIST_CSS: Asset = asset!("/assets/styling/idea_list.css");

/// Component to display all submitted ideas
#[component]
pub fn IdeaList(refresh_trigger: Signal<u32>) -> Element {
    // Use use_resource to fetch ideas from server
    let ideas = use_resource(move || async move {
        // Re-run when refresh_trigger changes
        let _ = refresh_trigger();
        get_all_ideas_server().await
    });

    rsx! {
        document::Link { rel: "stylesheet", href: IDEA_LIST_CSS }

        div {
            id: "idea-list-container",
            h2 { "Submitted Ideas" }

            match ideas() {
                Some(Ok(ideas_vec)) => rsx! {
                    if ideas_vec.is_empty() {
                        p { class: "empty-state", "No ideas submitted yet. Be the first!" }
                    } else {
                        for idea in ideas_vec {
                            div {
                                class: "idea-card",
                                h3 { "{idea.title}" }
                                p { class: "description", "{idea.description}" }
                                if !idea.tags.is_empty() {
                                    div {
                                        class: "tags",
                                        for tag in idea.tags {
                                            span { class: "tag", "{tag}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    p { class: "error", "Failed to load ideas: {e}" }
                },
                None => rsx! {
                    p { class: "loading", "Loading ideas..." }
                }
            }
        }
    }
}
