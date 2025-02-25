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
        let target_name = $name;
        let mut components = Vec::with_capacity(32);

        fn traverse<'a>(
            value: &'a serde_json::Value,
            list: &mut Vec<serde_json::Value>,
            name: &str,
            markdown_buffer: &mut String,
        ) {
            if let serde_json::Value::Object(obj) = value {
                if let Some(component) = obj.get("component") {
                    if component == name {
                        if name == "TextContent" {
                            if let Some(text) = obj.get("text").and_then(|t| t.as_str()) {
                                markdown_buffer.clear();
                                let parser = pulldown_cmark::Parser::new(text);
                                pulldown_cmark::html::push_html(markdown_buffer, parser);
                                let mut new_obj = obj.clone();
                                new_obj.insert(
                                    "text".to_string(),
                                    serde_json::Value::String(markdown_buffer.clone()),
                                );
                                list.push(serde_json::Value::Object(new_obj));
                            }
                        } else {
                            list.push(value.clone());
                        }
                    }
                }
                for v in obj.values() {
                    traverse(v, list, name, markdown_buffer);
                }
            } else if let serde_json::Value::Array(arr) = value {
                for item in arr {
                    traverse(item, list, name, markdown_buffer);
                }
            }
        }

        let mut markdown_buffer = String::with_capacity(1024);
        traverse($data, &mut components, target_name, &mut markdown_buffer);
        components
    }};
}

#[macro_export]
macro_rules! get_data {
    ({ $param:ident: $value:expr }) => {{
        let token = std::env::var("ST_TOKEN")
            .unwrap_or_else(|_| panic!("ST_TOKEN environment variable not set"));
        let base_url = std::env::var("ST_BASE_URL")
            .unwrap_or_else(|_| panic!("ST_BASE_URL environment variable not set"));

        let (url, field, filter_value) = match stringify!($param) {
            "slug" => (
                format!("{}/{}/?token={}", base_url, $value, token),
                "story",
                None,
            ),
            "starts_with" => (
                format!("{}?starts_with={}&token={}", base_url, $value, token),
                "stories",
                Some($value),
            ),
            _ => panic!("Unsupported parameter: {}", stringify!($param)),
        };

        let mut response = ureq::get(&url)
            .call()
            .unwrap_or_else(|e| panic!("Failed to send request to {}: {}", url, e));

        let body = response
            .body_mut()
            .read_to_string()
            .unwrap_or_else(|_| panic!("Failed to read response body"));

        let json: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or_else(|_| panic!("Failed to parse JSON from {}", url));

        let mut data = match json.get(field) {
            Some(value) => value.clone(),
            None => serde_json::Value::Array(vec![]),
        };

        if let Some(filter_value) = filter_value {
            if let serde_json::Value::Array(ref mut arr) = data {
                arr.retain(|item| {
                    item.get("full_slug")
                        .and_then(|slug| slug.as_str())
                        .map(|slug| slug != &format!("{}/", filter_value))
                        .unwrap_or(false)
                });
            }
        }

        data
    }};
}
