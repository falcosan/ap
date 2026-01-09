use std::{ env, sync::LazyLock };

pub static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::new_with_config(
        ureq::config::Config
            ::builder()
            .max_idle_connections_per_host(10)
            .max_idle_age(std::time::Duration::from_secs(90))
            .build()
    )
});

pub static ST_TOKEN: LazyLock<String> = LazyLock::new(||
    env::var("ST_TOKEN").expect("ST_TOKEN not set")
);
pub static ST_BASE_URL: LazyLock<String> = LazyLock::new(||
    env::var("ST_BASE_URL").expect("ST_BASE_URL not set")
);
