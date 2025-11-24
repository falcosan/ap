use crate::environment::{ PageContext, ENV };

pub fn home(current_path: &str) -> String {
    ENV.render_template("home.html", PageContext {
        data: extract_components!(&get_data!({ slug: "home" }), "TextContent"),
        current_path: current_path.to_string(),
    })
}
