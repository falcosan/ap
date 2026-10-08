use crate::environment::render;
use crate::http::{story, text_contents};
use axum::http::StatusCode;
use pulldown_cmark::{Parser, html};

pub async fn home(current_path: &str) -> Result<String, StatusCode> {
    let content = story(&["home"]).await?;
    let sections: Vec<String> = text_contents(&content)
        .into_iter()
        .map(markdown_to_html)
        .collect();
    render("home.html", current_path, sections)
}

fn markdown_to_html(markdown: &str) -> String {
    let mut output = String::new();
    html::push_html(&mut output, Parser::new(markdown));
    output
}
