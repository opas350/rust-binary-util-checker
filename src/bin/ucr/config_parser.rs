use config::Config;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Proxy{
    url: String
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    debug: bool,
    proxy: Proxy,
    #[serde(rename = "repos")]
    repos: Vec<String>
}

pub fn make() -> Config {
    let mut configuration_parsed = config::Config::default();
    configuration_parsed
        .merge(config::File::with_name("Config"))
        .unwrap()
        .to_owned()
}

