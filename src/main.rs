mod app;
mod components;
mod pages;
mod route;

use app::App;

fn main() {
    dioxus::launch(App);
}
