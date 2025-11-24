use std::sync::LazyLock;

pub static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::new_with_config(
        ureq::config::Config::builder()
            .max_idle_connections_per_host(10)
            .max_idle_age(std::time::Duration::from_secs(90))
            .build(),
    )
});
