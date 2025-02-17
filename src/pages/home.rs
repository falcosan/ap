use crate::components::Dogs;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Dogs {}
    }
}
