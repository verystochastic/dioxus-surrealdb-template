use crate::components::{IdeaForm, IdeaList};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // Signal to trigger list refresh when a new idea is submitted
    let mut refresh_trigger = use_signal(|| 0u32);
    // Signal to control form visibility
    let mut show_form = use_signal(|| false);

    rsx! {
        div {
            id: "ideas-section",

            // Show "Add Idea" button when form is hidden
            if !show_form() {
                button {
                    r#type: "button",
                    class: "add-idea-btn",
                    onclick: move |_| show_form.set(true),
                    "add idea"
                }
            }

            // Show form when toggled on
            if show_form() {
                IdeaForm {
                    on_submit_success: move |_| {
                        *refresh_trigger.write() += 1;
                        show_form.set(false);
                    },
                    on_cancel: move |_| {
                        show_form.set(false);
                    }
                }
            }

            IdeaList {
                refresh_trigger: refresh_trigger,
                on_delete_success: move |_| {
                    *refresh_trigger.write() += 1;
                }
            }
        }
    }
}
