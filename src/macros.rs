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
        use serde_json::Value;

        fn traverse(value: Value, list: &mut Vec<Value>, name: &str) {
            match value {
                Value::Object(mut obj) => {
                    if obj.get("component").map(|c| c == name).unwrap_or(false) {
                        if name == "TextContent" {
                            let converted = obj.get("text").and_then(|t| t.as_str()).map(|text| {
                                let mut buf = String::with_capacity(text.len() * 2);
                                html::push_html(&mut buf, Parser::new(text));
                                buf
                            });
                            if let Some(converted) = converted {
                                obj.insert("text".to_string(), Value::String(converted));
                                list.push(Value::Object(obj));
                                return;
                            }
                        } else {
                            list.push(Value::Object(obj.clone()));
                        }
                    }
                    for (_, v) in obj {
                        traverse(v, list, name);
                    }
                }
                Value::Array(arr) => {
                    for item in arr {
                        traverse(item, list, name);
                    }
                }
                _ => {}
            }
        }

        let mut components = Vec::with_capacity(32);
        traverse($data, &mut components, $name);
        components
    }};
}

#[macro_export]
macro_rules! get_data {
    ({ $param:ident: $value:expr }) => {{
        use serde_json::Value;
        use $crate::http::{fetch_json, ST_BASE_URL, ST_TOKEN};

        let value_str: String = $value.into();
        let (url, field, filter) = match stringify!($param) {
            "slug" => (
                format!("{}/{}?token={}", *ST_BASE_URL, value_str, *ST_TOKEN),
                "story",
                None,
            ),
            "starts_with" => (
                format!(
                    "{}?starts_with={}&token={}",
                    *ST_BASE_URL, value_str, *ST_TOKEN
                ),
                "stories",
                Some(value_str),
            ),
            _ => panic!("Unsupported parameter: {}", stringify!($param)),
        };

        match fetch_json(url).await {
            Some(mut json) => {
                let mut data = json
                    .as_object_mut()
                    .and_then(|o| o.remove(field))
                    .unwrap_or(Value::Array(Vec::new()));
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
