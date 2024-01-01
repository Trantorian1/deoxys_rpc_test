mod api_key;
use api_key::load_api_key;

use jsonrpsee::rpc_params;
use serde::Deserialize;
use jsonrpsee::http_client::HttpClientBuilder;
use rpc_call::RpcCall;

#[derive(Deserialize, Debug)]
struct BlockData {
    block_hash: String,
    block_number: u32,
}

impl RpcCall for BlockData {}

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