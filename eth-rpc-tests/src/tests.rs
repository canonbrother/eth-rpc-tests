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
fn should_eth_accounts() {
  let req = json_req!("eth_accounts", json!([]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

// #[test]
// fn should_eth_coinbase() {
//   let req = json_req!("eth_coinbase", json!([]));
//   let res = post_rpc(req, &env::var("RPC_URL").unwrap());
//   assert!(res.get("error").is_none());
// }

#[test]
fn should_eth_gasPrice() {
  let req = json_req!("eth_gasPrice", json!([]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_getBalance() {
  let req = json_req!("eth_getBalance", json!(["0xaaaf5374fce5edbc8e2a8697c15331677e6ebaaa", "latest"]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_getBalance_pending() {
  let req = json_req!("eth_getBalance", json!(["0xaaaf5374fce5edbc8e2a8697c15331677e6ebaaa", "pending"]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_getBlockByHash() {
  let req = json_req!("eth_getBlockByHash", json!(["0x75e65fb3bbf5f53afe26dcc72df6a95b0e8ca5f1c450145d8c3915bd0308b75b",false]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_getBlockByNumber() {
  let req = json_req!("eth_getBlockByNumber", json!(["0x0", false]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

// #[test]
// fn should_eth_getBlockTransactionCountByHash() {
//   let req = json_req!("eth_getBlockTranasctionCountByHash", json!([]));
//   let res = post_rpc(req, &env::var("RPC_URL").unwrap());
//   assert!(res.get("error").is_none());
// }

// #[test]
// fn should_eth_getBlockTransactionCountByNumber() {
//   let req = json_req!("eth_getBlockTranasctionCountByNumber", json!([]));
//   let res = post_rpc(req, &env::var("RPC_URL").unwrap());
//   assert!(res.get("error").is_none());
// }

#[test]
fn should_eth_blockNumber() {
  let req = json_req!("eth_blockNumber", json!([]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

// #[test]
// fn should_eth_call() {
//   let req = json_req!("eth_call", json!([{
//     "from": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
//     "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
//     "gas": "0x76c0",
//     "gasPrice": "0x9184e72a000",
//     "value": "0x9184e72a",
//     "data": "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"
//   }, "latest"]));
//   let res = post_rpc(req, &env::var("RPC_URL").unwrap());
//   // assert!(res.get("error").is_none()); // insufficient funds for ...
// }

#[test]
fn should_eth_chainId() {
  let req = json_req!("eth_chainId", json!([]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

// #[test]
// fn should_eth_estimateGas() {
//   let req = json_req!("eth_estimateGas", json!([{
//     "from": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
//     "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
//     "gas": "0x76c0",
//     "gasPrice": "0x9184e72a000",
//     "value": "0x9184e72a",
//     "data": "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"
//   }, "latest"]));
//   let res = post_rpc(req, &env::var("RPC_URL").unwrap());
//   // assert!(res.get("error").is_none()); // insufficient funds for transfer
// }

#[test]
fn should_eth_getCode() {
  let req = json_req!("eth_getCode", json!(["0x0000000000000000000000000000000000000001", "latest"]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

// #[test]
// fn should_eth_getFilterChanges() {
//   let req = json_req!("eth_getFilterChanges", json!(["0x0"]));
//   let res = post_rpc(req, &env::var("RPC_URL").unwrap());
//   assert!(res.get("error").is_none());
// }

#[test]
fn should_eth_getLogs() {
  let req = json_req!("eth_getLogs", json!([{
    "limit":1,
    "blockHash":"0x22ceeddb4a1d321ab3660c5497f04dfd78c05207fba6e48f3bc23ac82747243b"
  }]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_getProof() {
  let req = json_req!("eth_getProof", json!(["0xaaaf5374fce5edbc8e2a8697c15331677e6ebaaa", [], "latest"]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_getStorageAt() {
  let req = json_req!("eth_getStorageAt", json!(["0x0000000000000000000000000000000000000001", "0x4", "latest"]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

// #[test]
// fn should_eth_getTransactionCount() {
// 	let secret = "8a283037bb19c4fed7b1c569e40c7dcff366165eb869110a1b11532963eb9cb2".parse().unwrap();
//   let req = json_req!("eth_getTransactionCount", json!(["", "latest"]));
//   let res = post_rpc(req, &env::var("RPC_URL").unwrap());
//   assert!(res.get("error").is_none());
// }
  
#[test]
fn should_eth_hashrate() {
  let req = json_req!("eth_hashrate", json!([]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_newBlockFilter() {
  let req = json_req!("eth_newBlockFilter", json!([]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_getTransactionReceipt() {
  let req = json_req!("eth_getTransactionReceipt", json!(["0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238"]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

#[test]
fn should_eth_protocolVersion() {
  let req = json_req!("eth_protocolVersion", json!([]));
  let res = post_rpc(req, &env::var("RPC_URL").unwrap());
  assert!(res.get("error").is_none());
}

// #[test]
// fn should_eth_submitHashrate() {
//   let req = json_req!("eth_submitHashrate", json!([
//     "0x0000000000000000000000000000000000000000000000000000000000500000",
// 		"0x59daa26581d0acd1fce254fb7e85952f4c09d0915afd33d3886cd914bc7d283c"
//   ]));
//   let res = post_rpc(req, &env::var("RPC_URL").unwrap());
//   println!("res: {:?}", res);
//   // assert!(res.get("error").is_none()); // the method does not exists ???
// }

// #[test]
// fn should_eth_sendRawTransaction() {
//   let req = json_req!("eth_sendRawTransaction", json!([]));
//   let res = post_rpc(req, &env::var("RPC_URL").unwrap());
//   assert!(res.get("error").is_none());
// }

