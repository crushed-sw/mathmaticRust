use redis::{Client, AsyncCommands};
use redis::aio::MultiplexedConnection;
use crate::util::error::Error;

pub struct RedisService {
    pub client: Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        let client = Client::open(url).unwrap();
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<MultiplexedConnection, Error> {
        let conn = self.client.get_multiplexed_async_connection().await;
        if conn.is_err() {
            return Err(Error::from("redis 连接出错"));
        }
        Ok(conn.unwrap())
    }

    pub async fn del_string(&self, key: &str) -> Result<(), Error> {
        let mut conn = self.get_conn().await?;
        let res = conn.del::<&str, ()>(key).await;
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from("操作失败")),
        }
    }

    pub async fn get_string(&self, key: &str) -> Result<String, Error> {
        let mut conn = self.get_conn().await?;
        let value = conn.get(key).await;
        match value {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::from("操作失败")),
        }
    }

    pub async fn set_string_ex(&self, key: &str, value: &str, time: u64) -> Result<(), Error> {
        let mut conn = self.get_conn().await?;
        let res = conn.set_ex::<&str, &str, ()>(key, value, time).await;
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::from("操作失败")),
        }
    }
}
