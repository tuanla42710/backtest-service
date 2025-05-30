use serde::de::StdError;
use sqlx::{Connection, ConnectOptions};
use sqlx::mysql::{MySqlConnectOptions, MySqlConnection, MySqlPool, MySqlSslMode};


pub struct Database;

impl Database {
    pub async fn initialize_pool() -> Result<MySqlPool, Box<dyn StdError>> {
        let database_url = "mysql://root:123456789@localhost/rust";
        let pool = MySqlPool::connect(database_url).await?;
        Ok(pool)
    }
}