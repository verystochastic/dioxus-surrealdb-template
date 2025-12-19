use crate::server_functions::submit_idea_server;
use dioxus::prelude::*;

const IDEA_FORM_CSS: Asset = asset!("/assets/styling/idea_form.css");

/// Form component for submitting new ideas
#[component]
pub fn IdeaForm(on_submit_success: EventHandler<()>) -> Element {
    // State for form inputs
    let mut title = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut tags_input = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);
    let mut success_message = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: IDEA_FORM_CSS }

        div {
            id: "idea-form-container",
            h2 { "submit your idea" }

            form {
                onsubmit: move |event| async move {
                    event.prevent_default();
                    is_submitting.set(true);

                    // Parse tags from comma-separated input
                    let tags: Vec<String> = tags_input()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();

                    // Call server function
                    match submit_idea_server(title(), description(), tags).await {
                        Ok(_) => {
                            success_message.set("idea submitted successfully".to_string());
                            // Clear form
                            title.set(String::new());
                            description.set(String::new());
                            tags_input.set(String::new());
                            // Notify parent component
                            on_submit_success.call(());
                        }
                        Err(e) => {
                            success_message.set(format!("error: {}", e));
                        }
                    }

                    is_submitting.set(false);
                },

                div {
                    class: "form-field",
                    label { "title" }
                    input {
                        r#type: "text",
                        value: "{title}",
                        oninput: move |e| title.set(e.value()),
                        required: true,
                    }
                }

                div {
                    class: "form-field",
                    label { "description" }
                    textarea {
                        value: "{description}",
                        oninput: move |e| description.set(e.value()),
                        rows: 4,
                        required: true,
                    }
                }

                div {
                    class: "form-field",
                    label { "tags (comma-separated)" }
                    input {
                        r#type: "text",
                        value: "{tags_input}",
                        oninput: move |e| tags_input.set(e.value()),
                    }
                }

                button {
                    r#type: "submit",
                    disabled: is_submitting(),
                    class: "submit-btn",
                    "submit idea"
                }
            }

            if !success_message().is_empty() {
                p { class: "message", "{success_message}" }
            }
        }
    }
}
