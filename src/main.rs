mod model;
mod database;
mod indicators;
mod order;
mod portfolio;

use crate::model::trading_data;
use std::io;
use std::fs;
use polars::prelude::*;
use std::path::Path;
use serde::de::StdError;
use sqlx::mysql::MySqlPool;
use crate::indicators::trend::trend_indicator;
use crate::indicators::momentum::momentum_indicator;
use crate::indicators::utils::utils::rolling;
use crate::order::order::{Order, OrderType};
use crate::portfolio::portfolio::Portfolio;

fn read_csv(path : &str) -> Result<DataFrame> {
    let mut file = CsvReader::from_path(path)
        ?.has_header(true)
        .finish().unwrap();
    return Ok(file);
}

pub async fn write_to_sql(data : &DataFrame, pool : &MySqlPool, ticket : String) -> std::result::Result<(), Box<dyn StdError>> {
    
    for i in 0..data.height(){
        let timestamp  = data.column("timestamp").unwrap().utf8().unwrap().get(i).unwrap().to_string();
        let open = data.column("open").unwrap().f64().unwrap().get(i).unwrap();
        let close = data.column("close").unwrap().f64().unwrap().get(i).unwrap();
        let high = data.column("high").unwrap().f64().unwrap().get(i).unwrap();
        let low = data.column("low").unwrap().f64().unwrap().get(i).unwrap();
        let volume = data.column("volume").unwrap().i64().unwrap().get(i).unwrap();

        let trading_data = trading_data::Ohcl {
            ticket : ticket.to_string(),
            timestamp : timestamp,
            open : open,
            close : close,
            high : high,
            low : low,
            volume : volume
        };
        trading_data.to_sql(pool).await?;
    }
    return Ok(());
}

pub async fn write_15m_data(){
    let pool = database::database::initial_connect().await.unwrap();

    let directory = "src/data/15M";

    let entries = fs::read_dir(directory).unwrap();
    let mut list_file : Vec<String> = vec![];
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file(){
            let temp = path.to_str().unwrap().to_string();
            list_file.push(temp);
        }
    }
    for file in &list_file {
        let mut data = read_csv(file).unwrap();
        write_to_sql(&mut data, &pool, String::from("IBM")).await.unwrap();
    }
}

pub async fn write_data_dly(){
    let path = "src/data/days/daily_IBM2013-2025.csv";
    
    let pool = database::database::initial_connect().await.unwrap();
    
    let data = read_csv(path).unwrap();

    write_to_sql(&data, &pool, String::from("IBM")).await.unwrap()
}


async fn write_csv_basic(df: &mut DataFrame) -> Result<()> {
    let mut file = std::fs::File::create("output.csv")?;
    CsvWriter::new(&mut file)
        .has_header(true)  
        .finish(df)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let ticket = String::from("IBM");
    let ibm = trading_data::TradingData{ticket : String::from("IBM")};
    let mut df = ibm.load_data().await.unwrap();
    df = df.sort(["timestamp"], false).unwrap();
    let n_rsi =  14;
    let n_ma = 20;
    let name = format!("MA_{}",n_ma);
    let mask = df.column("timestamp").unwrap().gt("2021-02-01").unwrap();
    df = df.filter(&mask).unwrap();
    momentum_indicator::rsi(&mut df, n_rsi);
    trend_indicator::ma(&mut df, n_ma);
    let rsi_buy_signal = df.column("rsi").unwrap().gt(30).unwrap() & df.column("rsi").unwrap().lt(60).unwrap();
    let ma_buy_signal = df.column(&name).unwrap().gt(df.column("close").unwrap()).unwrap();

    let rsi_sell_signal = df.column("rsi").unwrap().gt(70).unwrap();
    let ma_sell_signal= df.column(&name).unwrap().lt(df.column("close").unwrap()).unwrap();

    let buy_condition = &rsi_buy_signal & &ma_buy_signal;
    let sell_condition = &rsi_sell_signal & &ma_sell_signal;

    let mut signal = Series::new("sig",
                                 buy_condition.into_iter().zip(sell_condition.into_iter())
                                     .map(|(buy,sell)| match (buy, sell) {
                                         (Some(true),Some(false)) => 1,
                                         (Some(false), Some(true))  => 0,
                                         _ => -1
                                     }).collect::<Vec<i64>>());
    df.with_column(signal.clone());
    
    let mut portfolio = Portfolio {
        cash : 300000.0,
        stocks : Vec::new()
    };
    
    for i in 0..df.height(){
        let signal = df.column("sig").unwrap().i64().unwrap().get(i).unwrap();
        let timestamp = df.column("timestamp").unwrap().utf8().unwrap().get(i).unwrap();
        let close = df.column("close").unwrap().f64().unwrap().get(i).unwrap();
        match signal {
            1 => {
                let order = Order {
                    ticket : ticket.clone(),
                    price : close,
                    volume : 1000,
                    order_type : OrderType::BUY
                };
                order.make_order(&mut portfolio);
            }
            0 => {
                let order = Order {
                    ticket : ticket.clone(),
                    price : close,
                    volume : 1000,
                    order_type : OrderType::SELL
                };
                order.make_order(&mut portfolio);
            }
            _ => {
                &portfolio.update_price(ticket.clone(), close);
            }
        }
    }
    let pl = portfolio.nav();
    println!("{}", pl);
    
    

    
    
}






