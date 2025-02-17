use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
        }

        Outlet::<Route> {}
    }
}
