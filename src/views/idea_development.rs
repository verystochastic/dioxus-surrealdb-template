use crate::server_functions::{get_idea_by_id_server, update_idea_server};
use dioxus::prelude::*;

const IDEA_DEV_CSS: Asset = asset!("/assets/styling/idea_development.css");

#[component]
pub fn IdeaDevelopment(id: String) -> Element {
    // Load idea data
    let idea_data = use_resource(move || {
        let id = id.clone();
        async move { get_idea_by_id_server(id).await }
    });

    // Local state for editing
    let mut what_must_be_true = use_signal(|| Vec::<String>::new());
    let mut development_notes = use_signal(|| String::new());
    let mut new_statement = use_signal(|| String::new());
    let mut is_saving = use_signal(|| false);

    // Initialize local state when data loads
    use_effect(move || {
        if let Some(Ok(idea)) = idea_data.read().as_ref() {
            what_must_be_true.set(idea.what_must_be_true.clone());
            development_notes.set(idea.development_notes.clone());
        }
    });

    // Auto-save function
    let auto_save = move || {
        if let Some(Ok(idea)) = idea_data.read().as_ref() {
            let id = idea.id.clone().unwrap_or_default();
            let title = idea.title.clone();
            let description = idea.description.clone();
            let tags = idea.tags.clone();
            let wmbt = what_must_be_true();
            let notes = development_notes();

            spawn(async move {
                is_saving.set(true);
                let _ = update_idea_server(id, title, description, tags, wmbt, notes).await;
                is_saving.set(false);
            });
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: IDEA_DEV_CSS }

        div {
            class: "idea-development",

            match idea_data() {
                Some(Ok(idea)) => rsx! {
                    // Idea header (read-only)
                    div {
                        class: "idea-header",
                        h1 { "{idea.title}" }
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

                    // What must be true section
                    div {
                        class: "wmbt-section",
                        h2 { "what must be true?" }

                        // List of statements
                        if !what_must_be_true().is_empty() {
                            ul {
                                class: "wmbt-list",
                                for (idx, statement) in what_must_be_true().iter().enumerate() {
                                    li {
                                        class: "wmbt-item",
                                        key: "{idx}",

                                        input {
                                            r#type: "text",
                                            value: "{statement}",
                                            oninput: move |e| {
                                                let mut list = what_must_be_true();
                                                list[idx] = e.value();
                                                what_must_be_true.set(list);
                                                auto_save();
                                            }
                                        }

                                        button {
                                            r#type: "button",
                                            class: "delete-wmbt",
                                            onclick: move |_| {
                                                let mut list = what_must_be_true();
                                                list.remove(idx);
                                                what_must_be_true.set(list);
                                                auto_save();
                                            },
                                            "×"
                                        }
                                    }
                                }
                            }
                        }

                        // Add new statement
                        div {
                            class: "add-wmbt",
                            input {
                                r#type: "text",
                                placeholder: "Add a new statement...",
                                value: "{new_statement}",
                                oninput: move |e| new_statement.set(e.value())
                            }
                            button {
                                r#type: "button",
                                onclick: move |_| {
                                    if !new_statement().is_empty() {
                                        let mut list = what_must_be_true();
                                        list.push(new_statement());
                                        what_must_be_true.set(list);
                                        new_statement.set(String::new());
                                        auto_save();
                                    }
                                },
                                "+"
                            }
                        }
                    }

                    // Development notes section
                    div {
                        class: "notes-section",
                        h2 { "development notes" }
                        textarea {
                            value: "{development_notes}",
                            placeholder: "Write your detailed development notes here...",
                            oninput: move |e| {
                                development_notes.set(e.value());
                                auto_save();
                            }
                        }
                    }

                    // Saving indicator
                    if is_saving() {
                        p { class: "saving-indicator", "Saving..." }
                    }
                },
                Some(Err(e)) => rsx! {
                    p { class: "error", "Failed to load idea: {e}" }
                },
                None => rsx! {
                    p { class: "loading", "Loading idea..." }
                }
            }
        }
    }
}
