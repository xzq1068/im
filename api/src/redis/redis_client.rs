use anyhow::anyhow;
use redis::{AsyncCommands, Client, FromRedisValue, RedisResult};
use redis::aio::MultiplexedConnection;
use crate::init::API_CONFIG;

type Result<T> = anyhow::Result<T>;
#[derive(Clone)]
pub struct RedisOps {
    connection: MultiplexedConnection,
}

impl RedisOps {
    pub async fn connect() ->Result<RedisOps> {
        let address = format!("redis://:{}@{}/", API_CONFIG.redis.password, API_CONFIG.redis.address);

        let client = Client::open(address)?;

        let mut con = client.get_multiplexed_async_connection().await?;

        Ok(RedisOps{connection:con})
    }

    pub async fn set(&mut self, key: &str, value: &str) -> Result<()>
    {
        let res:RedisResult<()>=redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut self.connection)
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub async fn set_ex(&mut self, key: &str, value: &str, exp: u64) -> Result<()>
    {
        let res: RedisResult<()> = self.connection.set_ex(key, value, exp).await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }

    }

    pub async fn get<T:FromRedisValue>(&mut self, key: &str) -> Result<T>
    {
        let res:RedisResult<T>= self.connection.get(key).await;

        match res {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

}

#[cfg(test)]
mod tests{
    use crate::redis::redis_client::RedisOps;

    #[tokio::test]
    async fn test_connect() {
        let mut redis_ops = RedisOps::connect().await;

        match redis_ops {
            Ok(_) => {
                println!("连接成功");
            }
            Err(_) => {
                println!("连接失败");
            }
        }
    }

    #[tokio::test]
    async fn test_set() {
        let mut redis_ops = RedisOps::connect().await.unwrap();

        let res=redis_ops.set("k1", "xzq-rs").await;

        match res {
            Ok(_) => {
                println!("set成功");
            }
            Err(_) => {
                println!("set失败");
            }
        }
    }

    #[tokio::test]
    async fn test_set_exp() {
        let mut redis_ops = RedisOps::connect().await.unwrap();

        let res = redis_ops.set_ex("k1", "xzq-rs", 100).await;

        match res {
            Ok(_) => {
                println!("set成功");
            }
            Err(_) => {
                println!("set失败");
            }
        }
    }

    #[tokio::test]
    async fn test_get() {
        let mut redis_ops = RedisOps::connect().await.unwrap();

        let res= redis_ops.get::<std::string::String>("k1").await;

        match res {
            Ok(str) => {
                println!("get成功 value:{str}");
            }
            Err(e) => {
                println!("get失败 {:#?}", e);
            }
        }

    }



}