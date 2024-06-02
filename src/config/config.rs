use std::{collections::HashMap, fs};
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    pub global: Global,
    pub web: Web,
    pub mysql: Mysql,
    pub redis: Redis,
    pub log: Log,
    pub jwt: Jwt,
    pub time: Time,
    pub error: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Global {
    pub debug: Option<bool>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Web {
    pub name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<u16>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Mysql {
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub ip: Option<String>,
    pub url: Option<String>,
    pub pool_len: Option<u64>,
    pub pool_timeout: Option<u64>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Redis {
    pub url: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Log {
    pub dir: Option<String>,
    pub temp_size: Option<String>,
    pub log_type: Option<String>,
    pub chan_len: Option<u32>,
    pub pack_compress: Option<String>,
    pub rolling_type: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Jwt {
    pub secret: Option<String>,
    pub exp: Option<u32>,
    pub refresh_token: Option<u32>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Time {
    pub datetime_format: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let contents = fs::read_to_string("./config.toml")
                       .expect("Should have been able to read the file");
        let config: Config = toml::from_str(contents.as_str()).unwrap();

        if cfg!(debug_assertions) {
            println!("[abs_admin] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[abs_admin] ///////////////////// Start On Release Mode ////////////////////////////");
        };

        config
    }
}

impl Config {
    pub fn get_database_url(&self) -> String {
        match &self.mysql.url {
            Some(value) => String::from(value),
            None => {
                match [&self.mysql.user, &self.mysql.password, &self.mysql.database, &self.mysql.ip] {
                    [Some(user), Some(password), Some(database), Some(ip)] => format!("mysql://{}:{}@{}/{}", user, password, database, ip),
                    _ => String::from(""),
                }
            }
        }
    }
}
