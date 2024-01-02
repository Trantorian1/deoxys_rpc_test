# Deoxis RPC - Unit testing library

_A simple Rust RPC test utility for the [Deoxis](https://github.com/KasarLabs/deoxys) Starknet node._

## Getting started

> ℹ️ Before you get started, make sure to create an Alchemy account [here](https://www.alchemy.com/starknet). You will also need to deploy a Deoxys node locally or use an already existing one.

For tests to work, you will need to specify an **alchemy api url** and **deoxys api url**. These must be stored in `test/secret.json`.

> ⚠️ Make sure to **never commit or share your api keys** in `test/secret.json`.

*secret.json format:*
```json
{
    "alchemy": "alchemy-node-url",
    "deoxys": "deoxys-node-url"
}
```

## Writing unit tests

Unit tests should be written inside of `/test/src/lib.rs`, but nothing stops you from creating your own module. Just make sure to import the necessary dependencies, which are:

```rust
use anyhow::*;
use serde::*;
use rpc_call::*;
use rpc_call_derive::*;
use rpc_test_attribute::*;
```

Tests consist of two parts:
1. The test config file, stored under `/test/unit` in JSON format.
2. The unit test specified in `/test/src/lib.rs`.

### In `test.json`

*test config file format:*
```json
{
    "cmd": "starknet_blockHashAndNumber",
    "arg": [],
    "block_range": {
        "start_inclusive": 0,
        "stop_inclusive": 1
    }
}
```

config file fields:
- `cmd`: the rpc command to query the node with.
- `arg`: the arguments used during the rpc call.
- *`block_range`* **(optional)**: the range of starknet blocks to run the unit test against.

### In `lib.rs`

You must provide a structure specifying the format of the rpc call return value as well as a test with the path to the required json test config file.

*example test:*
```rust
#[cfg(test)]
mod tests {
    use anyhow::*;
    use serde::*;
    use rpc_call::*;
    use rpc_call_derive::*;
    use rpc_test_attribute::*;

    #[derive(Deserialize, Debug, PartialEq, RpcCall)]
    struct BlockData {
        block_hash: String,
        block_number: u32,
    }

    #[rpc_test(BlockData, "./unit/test.json")]
    fn block_data_test() {}
}
```