mod config;

#[cfg(test)]
mod tests {
    use super::*;
    use jsonrpsee::http_client::HttpClientBuilder;
    use config::Config;
    use anyhow::Context;
    use serde::Deserialize;
    use std::{fs::File, io::Read};
    use rpc_call::RpcCall;
    use rpc_call_derive::RpcCall;

    #[derive(Deserialize)]
    struct TestInput {
        cmd: String,
        arg: Vec<String>,
    }

    #[derive(Deserialize)]
    struct TestData {
        input: TestInput,
        output: serde_json::Value,
    }

    impl TestData {
        fn new(path: &str) -> anyhow::Result<Self> {
            let mut file = File::open(path)?;
            let mut content = String::new();

            file.read_to_string(&mut content)?;

            let test_data: TestData = serde_json::from_str(&content)
                .with_context(|| format!("Could not deserialize test at into TestData"))?;

            Ok(test_data)
        }
    }

    #[derive(Deserialize, Debug, PartialEq, RpcCall)]
    struct BlockData {
        block_hash: String,
        block_number: u32,
    }

    // TODO: this currently compares result ot "output" in test.json. 
    // Should compare Deoxys and Alchemy instead rpc call results instead.
    // (Note that this would require having a Deoxis node running locally)
    #[tokio::test]
    async fn test() {
        // IDEA: must be a way to reduce boilerplate here. Macros maybe?
        let config = Config::new("./secret.json").unwrap();
        let alchemy = HttpClientBuilder::default().build(config.alchemy).unwrap();

        let path = "./unit/test.json";
        let test_data = TestData::new(path)
            .with_context(|| format!("Could not retrieve test data from {path}"))
            .unwrap();

        // FIXME: replace expected by call to deoxis
        let response: BlockData = BlockData::call(&alchemy, &test_data.input.cmd, test_data.input.arg).await
            .with_context(|| format!("Error waiting for rpc call response in test {path}"))
            .unwrap();
        let expected: BlockData = serde_json::from_value(test_data.output)
            .with_context(|| format!("Error waiting for rpc call response in test {path}"))
            .unwrap();

        assert_eq!(response, expected);
    }
}