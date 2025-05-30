use std::error::Error;
use tokio::net::TcpStream;
use urlencoding::encode;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Message;
use tungstenite::http::Uri;
use tungstenite::handshake::client::Request;
use futures_util::{Sink, SinkExt, StreamExt};
use sqlx::MySqlPool;
use crate::model::trading_data::TradingData;
use crate::model::trading_data::WsMessage;
type WsStream = WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>;
pub async fn connect() -> Result<(WsStream, tungstenite::handshake::client::Response), Box<dyn std::error::Error>> {
    let api_key = "d01hpk1r01qile5vojp0d01hpk1r01qile5vojpg";
    let host = "ws.finnhub.io";
    let encode_token = encode(api_key);

    let request = Request::builder()
        .uri({
            let uri_str = format!("wss://{}/?token={}", host, encode_token);
            uri_str.parse::<Uri>().expect("Failed to parse URI")
        })
        .header("Host", host)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", tungstenite::handshake::client::generate_key())
        .header("Origin", format!("https://{}", host))
        .body(())
        .unwrap();
    let (mut ws_stream, response) = connect_async(request).await?;
    return Ok(( ws_stream, response));
}

pub async fn get_data(mut ws_stream: WsStream, ticket : String, pool : &MySqlPool) -> Result<(), Box<dyn std::error::Error>>  {
    print!("hello");
    let payload = serde_json::json!(
        {
            "type":"subscribe",
            "symbol":ticket
        }
    );
    if let Err(e) = ws_stream.send(Message::Text(payload.to_string())).await {
        eprintln!("Error sending message: {:?}", e);
        return Ok(());
    };
    
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(message)) => {
                match serde_json::from_str::<WsMessage>(&message) {
                    Ok(parsed) => {
                        match parsed.msg_type.as_str(){
                            "trade" => {
                                println!("{:?}", parsed);
                                for i in parsed.data {
                                    i.insert(pool).await.expect("fail when insert");
                                }
                            },
                            "ping" => {
                                println!("received ping");
                            },
                            _ => {
                                println!("Unknown type") }
                            }
                        }
                    Err(e) => {
                        eprintln!("Error parsing message: {:?}", e);
                    }
                    
                    _ => println!("Unknown error {}", message) 
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
            }
            _ => println!("Unknown message " )
        }
    }
    Ok(())
    
}