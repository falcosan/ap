use crate::components::Echo;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Echo {}
    }
}
