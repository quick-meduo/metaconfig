#[macro_use]
extern crate quote;

pub mod traits;
#[macro_use]
pub mod directive;

#[cfg(test)]
mod tests {
    use super::directive::MetaConfig;

    #[derive(MetaConfig)]
    pub struct Config {
        #[value(from = "DB_HOST")]
        pub db_host: String,

        #[value(from = "DB_PORT", default = "5432")]
        pub db_port: u16,
    }

    #[test]
    fn test_config_can_be_loaded_from_hashmap() {

        // Initialize config from a HashMap to avoid test race conditions
        let config = Config::init().unwrap();

        assert_eq!(config.db_host, "127.0.0.1");
        assert_eq!(config.db_port, 5432);
    }
}
