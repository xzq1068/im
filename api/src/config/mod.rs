use std::fs;
use serde::Deserialize;



#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub server: Server,
    pub datasource: Datasource,
    pub redis: Redis,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct Datasource {
    pub db_name: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub pool_max_connection: u32,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub address: String,
    pub password: String,
}

pub(super) fn load_config()->ApiConfig {

    let config_str = fs::read_to_string("config.toml").expect("❌ load config error");

    let config: ApiConfig = toml::from_str(&config_str).expect("❌ parse config error");

    config
}


#[cfg(test)]
mod tests {
    use crate::config::load_config;

    #[test]
    fn test_load_config() {
        let api_config = load_config();

        println!("{:#?}", api_config);
    }
}