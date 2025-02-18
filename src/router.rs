use web_sys::{window, Location};

pub fn get_window_location() -> Option<Location> {
    window().map(|w| w.location())
}

pub fn render_page<F>(page: F)
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
