use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

pub fn about() -> String {
    #[derive(Serialize)]
    struct Props {
        content: String,
    }

    let env = ENV.lock().unwrap();

    let template = env.get_template("about.html").unwrap();

    let page = Props {
        content: "About".into(),
    };

    template.render(context!(page)).unwrap()
}
