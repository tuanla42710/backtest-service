mod database;
mod model;
mod socket;

use crate::database::JdbcUtil;
use crate::model::trading_data;
use crate::socket::connect;
use futures_util::future;
use futures_util::{Sink, SinkExt, StreamExt};
use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

type WsStream = WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>;

#[tokio::main]
async fn main() {
    let pool = JdbcUtil::Database::initialize_pool().await.unwrap();
    let a = connect::connect().await;
    println!("sc");
    match a {
        Ok((mut ws_stream, response)) => {
            println!("ws_stream");
            connect::get_data(ws_stream,String::from("BINANCE:BTCUSDT"), &pool).await;
        }
        Err(e) => {println!("{:#?}", e);}
        _ => println!("unknown error")
    }
    
}
