use std::env;

const DEFAULT_PATH: &str = "conf/application.yaml";

#[derive(Debug)]
pub struct Address {
    host: String,
    port: u16,
}

#[derive(Debug)]
pub struct Config {
    server: Address,
    accounts: Address,
}

impl Config {
    fn from_cfg(cfg: &config::Config) -> Config {
        let server_host = cfg.get::<String>("server.host").unwrap();
        let server_port = cfg.get::<u16>("server.port").unwrap();
        let accounts_host = cfg.get::<String>("accounts.host").unwrap();
        let accounts_port = cfg.get::<u16>("accounts.port").unwrap();

        Config {
            server: Address {
                host: server_host,
                port: server_port,
            },
            accounts: Address {
                host: accounts_host,
                port: accounts_port,
            },
        }
    }

    fn from_env() -> Config {
        let server_host = env::var("SERVER_HOST").unwrap();
        let server_port: u16 = env::var("SERVER_PORT").unwrap().parse().unwrap();
        let accounts_host = env::var("ACCOUNTS_HOST").unwrap();
        let accounts_port: u16 = env::var("ACCOUNTS_PORT").unwrap().parse().unwrap();

        Config {
            server: Address {
                host: server_host,
                port: server_port,
            },
            accounts: Address {
                host: accounts_host,
                port: accounts_port,
            },
        }
    }

    fn from_default_file() -> Config {
        let file = config::File::with_name(&DEFAULT_PATH);
        let mut cfg = config::Config::new();
        cfg.merge(file).unwrap();
        Config::from_cfg(&cfg)
    }

    pub fn read() -> Config {
        if env::var("MODE").is_ok() {
            Config::from_env()
        } else {
            Config::from_default_file()
        }
    }
}
