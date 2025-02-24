use crate::environment::ENV;

pub fn fallback() -> String {
    ENV.render_template("fallback.html", ())
}
