use crate::components::{IdeaForm, IdeaList};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // Signal to trigger list refresh when a new idea is submitted
    let mut refresh_trigger = use_signal(|| 0u32);

    rsx! {
        div {
            id: "ideas-section",
            IdeaForm {
                on_submit_success: move |_| {
                    *refresh_trigger.write() += 1;
                }
            }
            IdeaList {
                refresh_trigger: refresh_trigger
            }
        }
    }
}
