extern crate redis;
extern crate base64;
//extern crate uuid;

//use uuid::Uuid;
use serde_json::json;
use serde_json::Value;
use std::time::{SystemTime};
use std::str;

use redis::Commands;
pub struct MessageBusClient {
    connection: redis::Connection
}

impl MessageBusClient {

    pub fn new(port: i32) -> MessageBusClient {
        let url = format!("{}{}", "redis://127.0.0.1:", port);
        let client = redis::Client::open(url).unwrap();
        MessageBusClient {
            connection: client.get_connection().unwrap()
        }
    }

    pub fn prepare (&self, command: &str, expiration: i64) -> serde_json::Value {
        return json!({
            "ver": 1,
            "uid": "",
            "cmd": command,
            "exp": expiration,
            "dat": "",
            "src": 0,
            "dst": 0,
            "ret": "Uuid::new_v4()",
            "try": 10,
            "shm": "",
            "now": "(SystemTime::now() / 1000).floor()",
            "err": "",
        });
    }

    pub fn send(&mut self, mut message: Value, payload: &str) -> redis::RedisResult<()> {
        *message.get_mut("dat").unwrap() = json!(payload);
        let request = message.to_string();
        let _: () = self.connection.lpush("msgbus.system.local", request)?;
        Ok(())
    }

    
    pub fn read(&mut self, message: Value) -> Vec<serde_json::Value> {

        let mut responses: Vec<serde_json::Value> = Vec::new();
        while responses.len() < 1 {

            let redis_arg = message["ret"].to_string().into_bytes();
            //let redis_arg: str = *"msgbus.system.local"

            let results = self.connection.blpop(redis_arg, 5).expect("BLPOP failed");
            if let redis::Value::Bulk(vector) = results {
                if let redis::Value::Data(binary) = &vector[1] {
                    let response_json: Value = serde_json::from_str(str::from_utf8(&binary).unwrap()).unwrap();
                    responses.push(response_json)

                }
            }
            //response_json["dat"] = base64::decode(response_msg["dat"]);
        }
        return responses;
    }

}