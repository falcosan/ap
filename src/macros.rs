#[macro_export]
macro_rules! export {
  ($($module:ident),+) => {
      $(
          pub(crate) mod $module;
          pub(crate) use $module::$module;
      )+
  };
}

#[macro_export]
macro_rules! get_data {
    ({ $param:ident: $value:expr }) => {{
        const BASE_URL: &str = "https://api.storyblok.com/v2/cdn/stories";
        let token = std::env::var("ST_TOKEN")
            .unwrap_or_else(|_| panic!("ST_TOKEN environment variable not set"));

        let (url, field) = match stringify!($param) {
            "slug" => (format!("{}/{}/?token={}", BASE_URL, $value, token), "story"),
            "starts_with" => (
                format!("{}?starts_with={}&token={}", BASE_URL, $value, token),
                "stories",
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

        json.get(field)
            .unwrap_or_else(|| panic!("Missing '{}' field in response", field))
            .clone()
    }};
}
