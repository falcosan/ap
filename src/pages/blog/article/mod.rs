use crate::environment::{ PageContext, ENV };

pub fn article(current_path: &str, slug: &str) -> String {
    ENV.render_template("article.html", PageContext {
        data: get_data!({ slug: format!("blog/{}", slug) }),
        current_path: current_path.to_string(),
    })
}
