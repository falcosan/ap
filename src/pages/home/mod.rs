use crate::environment::ENV;
use minijinja::context;
use serde::Serialize;

#[derive(Serialize)]
struct PageContext<T> {
    data: T,
}

pub fn home() -> String {
    let env = ENV.lock().unwrap();
    let template = env.get_template("home.html").unwrap();
    let context = PageContext {
        data: "<h1>Kiosco ANTONIO</h1><br><img src=https://plutonphoto.wordpress.com/wp-content/uploads/2015/10/12138373_10207748081247843_5871218732533954674_o.jpg />",
    };
    template.render(context!(page => context)).unwrap()
}
