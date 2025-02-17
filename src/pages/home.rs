use crate::components::echo::Echo;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Echo {}
    }
}
