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
    ({ slug: $slug:expr }) => {{
        const BASE_URL: &str = "https://api.storyblok.com/v2/cdn/stories";
        let st_token = std::env::var("ST_TOKEN").expect("ST_TOKEN not set");
        let response = minreq::get(&format!("{}/{}?token={}", BASE_URL, $slug, st_token))
            .send()
            .expect("Request failed");
        let json: serde_json::Value = serde_json::from_str(
            response
                .as_str()
                .expect("Failed to convert response to string"),
        )
        .expect("Failed to parse JSON");

        json["story"].clone()
    }};
    ({ starts_with: $starts_with:expr }) => {{
        const BASE_URL: &str = "https://api.storyblok.com/v2/cdn/stories";
        let st_token = std::env::var("ST_TOKEN").expect("ST_TOKEN not set");
        let response = minreq::get(format!(
            "{}?starts_with={}&token={}",
            BASE_URL, $starts_with, st_token
        ))
        .send()
        .expect("Request failed");
        let json: serde_json::Value = serde_json::from_str(
            response
                .as_str()
                .expect("Failed to convert response to string"),
        )
        .expect("Failed to parse JSON");

        json["stories"].clone()
    }};
}
