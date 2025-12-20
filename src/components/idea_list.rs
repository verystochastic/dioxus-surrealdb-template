use crate::server_functions::{delete_idea_server, get_all_ideas_server};
use crate::Route;
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
                                // Header with title and action buttons
                                div {
                                    class: "idea-header",
                                    h3 { "{idea.title}" }

                                    div {
                                        class: "idea-actions",

                                        // Develop button (only if idea has an ID)
                                        if let Some(id) = &idea.id {
                                            {
                                                let id = id.to_owned();
                                                rsx! {
                                                    Link {
                                                        to: Route::IdeaDevelopment { id: id.clone() },
                                                        class: "develop-btn",
                                                        "→"
                                                    }
                                                }
                                            }
                                        }

                                        // Delete button (only if idea has an ID)
                                        if let Some(id) = &idea.id {
                                            {
                                                let id = id.to_owned();
                                                rsx! { button {
                                                r#type: "button",
                                                class: "delete-btn",
                                                onclick: move |evt| {
                                                evt.prevent_default();
                                                evt.stop_propagation();

                                                let id = id.clone();

                                                spawn(async move {
                                                    #[cfg(target_arch = "wasm32")]
                                                    web_sys::console::log_1(&format!("🔍 Delete clicked for ID: {}", id).into());

                                                    // Use native JavaScript confirm
                                                    #[cfg(target_arch = "wasm32")]
                                                    let confirmed = {
                                                        let window = web_sys::window().expect("no global window");
                                                        window.confirm_with_message("Delete this idea?").unwrap_or(false)
                                                    };

                                                    #[cfg(not(target_arch = "wasm32"))]
                                                    let confirmed = false;

                                                    #[cfg(target_arch = "wasm32")]
                                                    web_sys::console::log_1(&format!("🤔 Confirmed: {}", confirmed).into());

                                                    if confirmed {
                                                        #[cfg(target_arch = "wasm32")]
                                                        web_sys::console::log_1(&"✅ Calling delete_idea_server".into());

                                                        match delete_idea_server(id).await {
                                                            Ok(_) => {
                                                                #[cfg(target_arch = "wasm32")]
                                                                web_sys::console::log_1(&"🎉 Delete successful".into());
                                                                on_delete_success.call(());
                                                            }
                                                            Err(_e) => {
                                                                #[cfg(target_arch = "wasm32")]
                                                                web_sys::console::log_1(&format!("💥 Delete failed: {}", _e).into());
                                                            }
                                                        }
                                                    }
                                                });
                                            },
                                            "×"
                                        } }
                                            }
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
