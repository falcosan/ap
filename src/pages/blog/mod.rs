use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

pub fn blog() -> String {
    #[derive(Serialize)]
    struct Props {
        content: String,
    }

    let env = ENV.lock().unwrap();

    let template = env.get_template("blog.html").unwrap();

    let page = Props {
        content: "Blog".into(),
    };

    template.render(context!(page)).unwrap()
}
