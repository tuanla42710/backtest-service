
use sqlx::mysql::{MySqlPool};

pub async fn initial_connect() -> Result<MySqlPool, Box<dyn std::error::Error>> {
    let database_url = "mysql://root:123456789@localhost/rust";
    let pool = MySqlPool::connect(database_url).await?;
    Ok(pool)
}