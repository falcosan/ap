use crate::pages::{about, blog, home};
use web_sys::{console, window, Location};

fn get_window_location() -> Option<Location> {
    window().map(|w| w.location())
}

fn render_page<F>(page: F)
where
    F: FnOnce() -> String,
{
    if let Some(root) = window()
        .and_then(|w| w.document())
        .and_then(|doc| doc.get_element_by_id("root"))
    {
        root.set_inner_html(&page());
    }
}

pub fn router() {
    match get_window_location() {
        Some(location) => {
            let path = location.pathname().unwrap_or_default();
            match path.as_str() {
                "/blog" => render_page(blog),
                "/about" => render_page(about),
                _ => render_page(home),
            }
        }
        None => console::error_1(&"Failed to get window location".into()),
    }
}
