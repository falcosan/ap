export!(article);

use crate::environment::{ PageContext, ENV };

pub fn blog(current_path: &str) -> String {
    ENV.render_template("blog.html", PageContext {
        data: get_data!({ starts_with: "blog" }),
        current_path: current_path.to_string(),
    })
}
