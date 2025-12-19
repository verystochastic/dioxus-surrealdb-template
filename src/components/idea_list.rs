use crate::server_functions::{delete_idea_server, get_all_ideas_server};
use dioxus::document::eval;
use dioxus::prelude::*;

const IDEA_LIST_CSS: Asset = asset!("/assets/styling/idea_list.css");

/// Component to display all submitted ideas
#[component]
pub fn IdeaList(refresh_trigger: Signal<u32>, on_delete_success: EventHandler<()>) -> Element {
    // Use use_resource to fetch ideas from server
    let ideas = use_resource(move || async move {
        // Re-run when refresh_trigger changes
        let _ = refresh_trigger();
        get_all_ideas_server().await
    });

    // Delete handler with confirmation
    let handle_delete = move |idea_id: String| async move {
        // Debug: log the ID we're trying to delete
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Attempting to delete idea with ID: {}", idea_id).into());

        // Show browser confirmation dialog using eval
        let confirmed = match eval(r#"confirm("Delete this idea?")"#).recv::<bool>().await {
            Ok(val) => val,
            Err(_) => false,
        };

        if confirmed {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&"User confirmed deletion".into());

            // Call server function to delete
            match delete_idea_server(idea_id.clone()).await {
                Ok(_) => {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&"Delete successful, refreshing list".into());

                    // Notify parent to refresh the list
                    on_delete_success.call(());
                }
                Err(e) => {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&format!("Delete failed: {}", e).into());
                }
            }
        } else {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&"User cancelled deletion".into());
        }
    };

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
                                // Header with title and delete button
                                div {
                                    class: "idea-header",
                                    h3 { "{idea.title}" }
                                    // Delete button (only if idea has an ID)
                                    if let Some(ref id) = idea.id {
                                        button {
                                            class: "delete-btn",
                                            onclick: {
                                                let id = id.clone();
                                                move |_| {
                                                    let id = id.clone();
                                                    spawn(handle_delete(id));
                                                }
                                            },
                                            "×"
                                        }
                                    }
                                }
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
