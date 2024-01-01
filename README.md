# Deoxis RPC - Unit testing library

_A simple Rust RPC send & desirialization test utility._

The `RpcCall` trait is used to give structures deserialization capabilities.
Rpc calls are done using the `call` function.

> Make sure the structure's fields match the json fields of the RPC response.

## example
```rust
mod api_key;
use api_key::load_api_key;

use jsonrpsee::rpc_params;
use serde::Deserialize;
use jsonrpsee::http_client::HttpClientBuilder;
use rpc_call::RpcCall;
use rpc_call_derive::RpcCall;

#[derive(Deserialize, Debug, RpcCall)]
struct BlockData {
    block_hash: String,
    block_number: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = load_api_key("stark_rpc/secret.json")?;

    let client = HttpClientBuilder::default()
        .build(api_key)
        .unwrap();

    let response: BlockData = BlockData::call(&client, "starknet_blockHashAndNumber", rpc_params![]).await?;
    println!("response: {:?}", response);

    Ok(())
}
```