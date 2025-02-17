mod app;
mod route;
mod components {
    pub mod echo;
    pub mod navbar;
}
mod pages {
    pub mod home;
}

use app::App;

fn main() {
    dioxus::launch(App);
}
