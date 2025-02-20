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
    ($name:expr) => {{
        let st_token = std::env::var("ST_TOKEN").expect("ST_TOKEN not set");

        let response = minreq::get(format!(
            "https://api.storyblok.com/v2/cdn/stories/{}?token={}",
            $name, st_token
        ))
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
}
