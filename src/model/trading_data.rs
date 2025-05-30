use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::database::JdbcUtil;
use sqlx::MySqlConnection;
use futures_util::{Sink, SinkExt, StreamExt};
use serde::de::StdError;

use sqlx::mysql::MySqlPool;

#[derive(Debug, Deserialize)]
pub struct TradingData {
    c: Option<f64>,
    p : f64,
    s : String,
    t : u64,
    v: f64
}

#[derive(Debug, Deserialize)]
pub struct WsMessage {
    #[serde(default)]
    pub data : Vec<TradingData>,
    #[serde(rename = "type")]
    pub msg_type: String
}

impl TradingData {
    pub async fn insert(&self, pool: &MySqlPool) -> Result<(), Box<dyn StdError>> {
        sqlx::query(
            r#"
            INSERT INTO trading_data 
                ( close, price, symbol, timestamp, volume) 
            VALUES 
                (?, ?, ?, ?, ?)
            "#
        )
            .bind(&self.c)
            .bind(self.p)
            .bind(&self.s)
            .bind(self.t)
            .bind(self.v)
            .execute(pool)
            .await?;

        Ok(())
    }
    
    pub async fn batch_insert(
        pool: &MySqlPool,
        data: &[TradingData]
    ) -> Result<u64, Box<dyn StdError>> {
        let mut query = String::from(
            "INSERT INTO trading_data 
                ( close, price, symbol, timestamp, volume) 
             VALUES "
        );
        
        let placeholders = data.iter()
            .enumerate()
            .map(|(i, _)| if i == 0 { "(?, ?, ?, ?, ?)" } else { ", (?, ?, ?, ?, ?)" })
            .collect::<String>();

        query.push_str(&placeholders);
        
        let mut query = sqlx::query(&query);
        for record in data {
            query = query
                .bind(&record.c)
                .bind(record.p)
                .bind(&record.s)
                .bind(record.t)
                .bind(record.v);
        }

        let result = query.execute(pool).await?;
        Ok(result.rows_affected())
    }
}



