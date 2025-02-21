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

        let response = minreq::get(&url)
            .send()
            .unwrap_or_else(|_| panic!("Failed to send request to {}", url));

        let body = response
            .as_str()
            .unwrap_or_else(|_| panic!("Failed to read response body from {}", url));

        let json: serde_json::Value = serde_json::from_str(body)
            .unwrap_or_else(|_| panic!("Failed to parse JSON from {}", url));

        let mut data = json
            .get(field)
            .unwrap_or_else(|| panic!("Missing '{}' field in response", field))
            .clone();

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

#[macro_export]
macro_rules! extract_components {
    ($data:expr, $name:expr) => {{
        let mut components = Vec::new();

        fn traverse(value: &serde_json::Value, list: &mut Vec<serde_json::Value>, name: &str) {
            match value {
                serde_json::Value::Object(obj) => {
                    if let Some(component) = obj.get("component") {
                        if component.as_str() == Some(name) {
                            list.push(value.clone());
                        }
                    }
                    for (_, v) in obj {
                        traverse(v, list, name);
                    }
                }
                serde_json::Value::Array(arr) => {
                    for item in arr {
                        traverse(item, list, name);
                    }
                }
                _ => {}
            }
        }

        traverse($data, &mut components, $name);
        components
    }};
}
