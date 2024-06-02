pub mod chem_service;
pub mod user_service;
pub mod lettre_service;
pub mod redis_service;
pub mod article_service;
pub mod favorite_service;
pub mod comment_service;

pub use crate::config::config::Config;
use std::{collections::HashMap, time::Duration, fs, error::Error};
pub use rbatis::RBatis;

use once_cell::sync::Lazy;
use rbdc_mysql::MysqlDriver;

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::service::CONTEXT.rb
    };
}

pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());
pub struct ServiceContext {
    pub config: Config,
    pub rb: RBatis,
    pub redis_service: redis_service::RedisService,
    pub atom_real_surface_map: HashMap<String, String>,
    pub atom_real_points_map: HashMap<String, String>,
    pub atom_complex_map: HashMap<String, String>,
    pub orbit_map: HashMap<String, String>,
}

impl ServiceContext {
    pub async fn init_database(&self) {
        self.rb.init(MysqlDriver {}, self.config.get_database_url().as_str()).unwrap();
        let pool = self.rb.get_pool().unwrap();

        let db_pool_len: u64 = match self.config.mysql.pool_len {
            Some(pool_len) => pool_len,
            None => 32_u64,
        };
        let db_pool_timeout: u64 = match self.config.mysql.pool_timeout {
            Some(pool_timeout) => pool_timeout,
            None => 60_u64,
        };

        pool.set_max_open_conns(db_pool_len).await;
        pool.set_timeout(Some(Duration::from_secs(db_pool_timeout))).await;
    }

    fn update_file(dirname: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let dir = fs::read_dir(dirname)?;
        let mut map = HashMap::new();
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().into_string();
            let name = match name {
                Ok(value) => value,
                Err(_) => "".to_string(),
            };
            let string = fs::read_to_string(path)?;
            let name_vec: Vec<&str> = name.split(".").collect();

            map.insert(name_vec[0].to_string(), string);
        }

        Ok(map)
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = Config::default();
        let rb = RBatis::new();
        let redis_service = if let Some(ref redis_url) = config.redis.url {
            redis_service::RedisService::new(redis_url.as_str())
        } else {
            redis_service::RedisService::new("redis://127.0.0.1:6379")
        };

        let atom_real_surface_map = match Self::update_file("./static/atom/real/surface") {
            Ok(value) => value,
            Err(_) =>HashMap::new(),
        };
        let atom_real_points_map = match Self::update_file("./static/atom/real/points") {
            Ok(value) => value,
            Err(_) =>HashMap::new(),
        };

        let atom_complex_map = match Self::update_file("./static/atom/complex") {
            Ok(value) => value,
            Err(_) =>HashMap::new(),
        };
        let orbit_map = match Self::update_file("./static/orbit") {
            Ok(value) => value,
            Err(_) =>HashMap::new(),
        };

        ServiceContext {
            config,
            rb,
            redis_service,
            atom_real_surface_map,
            atom_real_points_map,
            atom_complex_map,
            orbit_map,
        }
    }
}
