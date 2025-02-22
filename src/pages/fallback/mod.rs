use crate::environment::ENV;

pub fn fallback() -> String {
    let template = &ENV.get_template("fallback.html");
    template.render(()).unwrap()
}
