use crate::environment::ENV;

pub fn fallback() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("fallback.html").unwrap();
    template.render(()).unwrap()
}
