#[macro_export]
macro_rules! export {
    ($($module:ident),+) => {
        $(
            pub mod $module;
            #[allow(unused_imports)]
            pub use $module::$module;
        )+
    };
}

#[macro_export]
macro_rules! extract_components {
    ($data:expr, $name:expr) => {{
        use pulldown_cmark::{html, Parser};
        use serde_json::{json, Map, Value};

        fn traverse(value: &Value, list: &mut Vec<Value>, name: &str, buf: &mut String) {
            match value {
                Value::Object(obj) => {
                    if obj.get("component").map(|c| c == name).unwrap_or(false) {
                        if name == "TextContent" {
                            if let Some(text) = obj.get("text").and_then(|t| t.as_str()) {
                                buf.clear();
                                html::push_html(buf, Parser::new(text));
                                let mut new_obj = Map::with_capacity(obj.len());
                                for (k, v) in obj {
                                    new_obj.insert(
                                        k.clone(),
                                        if k == "text" { json!(buf.as_str()) } else { v.clone() },
                                    );
                                }
                                list.push(Value::Object(new_obj));
                            }
                        } else {
                            list.push(value.clone());
                        }
                    }
                    for v in obj.values() {
                        traverse(v, list, name, buf);
                    }
                }
                Value::Array(arr) => {
                    for item in arr {
                        traverse(item, list, name, buf);
                    }
                }
                _ => {}
            }
        }

        let mut components = Vec::with_capacity(32);
        let mut buf = String::with_capacity(4096);
        traverse($data, &mut components, $name, &mut buf);
        components
    }};
}

#[macro_export]
macro_rules! get_data {
    ({ $param:ident: $value:expr }) => {{
        use serde_json::Value;
        use std::sync::LazyLock;

        static ST_TOKEN: LazyLock<String> =
            LazyLock::new(|| std::env::var("ST_TOKEN").expect("ST_TOKEN not set"));
        static ST_BASE_URL: LazyLock<String> =
            LazyLock::new(|| std::env::var("ST_BASE_URL").expect("ST_BASE_URL not set"));

        let value_str: String = $value.into();
        let (url, field, filter) = match stringify!($param) {
            "slug" => (
                format!("{}/{}?token={}", *ST_BASE_URL, value_str, *ST_TOKEN),
                "story",
                None,
            ),
            "starts_with" => (
                format!("{}?starts_with={}&token={}", *ST_BASE_URL, value_str, *ST_TOKEN),
                "stories",
                Some(value_str),
            ),
            _ => panic!("Unsupported parameter: {}", stringify!($param)),
        };

        match $crate::http::AGENT
            .get(&url)
            .call()
            .ok()
            .and_then(|r| r.into_body().read_json::<Value>().ok())
        {
            Some(json) => {
                let mut data = json.get(field).cloned().unwrap_or(Value::Array(vec![]));
                if let (Some(f), Value::Array(arr)) = (&filter, &mut data) {
                    let slug = format!("{}/", f);
                    arr.retain(|i| {
                        i.get("full_slug")
                            .and_then(|s| s.as_str())
                            .map(|s| s != slug)
                            .unwrap_or(false)
                    });
                }
                data
            }
            None => return $crate::environment::ENV.render_template("fallback.html", ()),
        }
    }};
}
