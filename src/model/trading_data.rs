use serde::de::StdError;
use crate::database::database;
use sqlx::mysql::{ MySqlPoolOptions};
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;
use sqlx::Row;
use sqlx::mysql::MySqlRow;
use polars::prelude::{Series, DataFrame,NamedFrom};
#[derive(Debug, FromRow)]
pub struct Ohcl {
    pub ticket : String,
    pub(crate) timestamp : String,
    pub open : f64,
    pub high : f64,
    pub low : f64,
    pub close : f64,
    pub volume : i64
}

impl Ohcl {
    pub async fn to_sql(&self, mysql_pool : &MySqlPool) -> Result<(), Box<dyn StdError>>{
        sqlx::query(
            r#"
                INSERT INTO trading_data_dly VALUES
                (? , ? , ? , ? , ? , ? , ?);
                "#
        )
            .bind(&self.ticket)
            .bind(&self.timestamp)
            .bind(&self.open)
            .bind(&self.high)
            .bind(&self.low)
            .bind(&self.close)
            .bind(&self.volume).execute(mysql_pool).await?;
        Ok(())

    }
}

pub struct TradingData {
    pub(crate) ticket : String
}

impl TradingData {

    pub async fn load_data(&self) -> Result<DataFrame, Box<dyn StdError>>{
        let pool = database::initial_connect().await.unwrap();
        let rows = sqlx::query(
            r"select * from trading_data_dly where ticket = ? "
        ).bind(&self.ticket).fetch_all(&pool).await.unwrap();

        let ticket : Vec<String> = rows.iter().map(|v| {
            v.get("ticket")
        }).collect();
        let timestamp : Vec<String> = rows.iter().map(|v| v.get("timestamp")).collect();
        let open : Vec<f64> = rows.iter().map(|v| v.get("open")).collect();
        let high : Vec<f64> = rows.iter().map(|v| v.get("high")).collect();
        let low  : Vec<f64> = rows.iter().map(|v| v.get("low")).collect();
        let close : Vec<f64> = rows.iter().map(|v| v.get("close")).collect();
        let volume : Vec<f64> = rows.iter().map(|v| v.get("volume")).collect();

        let df = DataFrame::new(vec![
            Series::new("ticket", ticket),
            Series::new("timestamp",timestamp),
            Series::new("open", open),
            Series::new("high", high),
            Series::new("low",low),
            Series::new("close",close),
            Series::new("volume", volume)
        ])?;
        
        Ok(df)
        
    }
    
    pub async fn _load_data(&self, transaction_date : String) -> Result<DataFrame, Box<dyn StdError>>{
        let pool = database::initial_connect().await.unwrap();

        let rows = sqlx::query(
            r"select * from trading_data_15m where ticket = ? and left(timestamp,10) = ?"
        )
            .bind(&self.ticket)
            .bind(&transaction_date)
            .fetch_all(&pool).await.unwrap();

        let ticket : Vec<String> = rows.iter().map(|v| {
            v.get("ticket")
        }).collect();
        let timestamp : Vec<String> = rows.iter().map(|v| v.get("timestamp")).collect();
        let open : Vec<f64> = rows.iter().map(|v| v.get("open")).collect();
        let high : Vec<f64> = rows.iter().map(|v| v.get("high")).collect();
        let low  : Vec<f64> = rows.iter().map(|v| v.get("low")).collect();
        let close : Vec<f64> = rows.iter().map(|v| v.get("close")).collect();
        let volume : Vec<f64> = rows.iter().map(|v| v.get("volume")).collect();

        let df = DataFrame::new(vec![
            Series::new("ticket", ticket),
            Series::new("timestamp",timestamp),
            Series::new("open", open),
            Series::new("high", high),
            Series::new("low",low),
            Series::new("close",close),
            Series::new("volume", volume)
        ])?;

        Ok(df)
    }

}