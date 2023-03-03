#![allow(non_snake_case)]

use std::env;
use reqwest::{self, header::CONTENT_TYPE};
use serde_json::{json, Value};

macro_rules! json_req {
  ($method: expr, $params: expr) => {{
      json!({
         "jsonrpc": "2.0",
         "id": 1,
         "method": $method,
         "params": $params,
      })
  }}
}

fn post_rpc(request: Value, url: &str) -> Value {
  let client = reqwest::blocking::Client::new();
  let response = client
      .post(url)
      .header(CONTENT_TYPE, "application/json")
      .body(request.to_string())
      .send()
      .unwrap();

  serde_json::from_str(&response.text().unwrap()).unwrap()
}

// RPC_URL="https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161" cargo test -- --nocapture

#[test]
fn should_eth_protocolVersion() {
  let req = json_req!("eth_protocolVersion", json!([]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_getBlockByNumber() {
  let req = json_req!("eth_getBlockByNumber", json!(["0x0", false]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}