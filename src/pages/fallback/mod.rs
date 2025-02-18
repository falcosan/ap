use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

pub fn fallback() -> String {
    #[derive(Serialize)]
    struct Props {
        content: String,
    }

    let env = ENV.lock().unwrap();

    let template = env.get_template("fallback.html").unwrap();

    let page = Props {
        content: "404".into(),
    };

    template.render(context!(page)).unwrap()
}
