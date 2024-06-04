use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

// The execute messages that the contract can handle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ChangeManager { new_manager: String },
    SubmitBlob { contents: ContentsMsg },
}

// The query messages that the contract can handle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetStateInfo {},
    GetBlobById { blob_id: u64 },
    GetBlobByBlock { terra_block_number: u64 },
}

// The structure for the contents of a blob submission message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ContentsMsg {
    pub action: String,
    pub blob_id: u64,
    pub message: Vec<String>,
}

// The structure for a single blob's data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Blob {
    pub blob_id: u64,
    pub terra_block_number: u64,
    pub terra_previous_block: u64,
    pub data: Vec<String>, // Array of base64-encoded-data Strings
}

// The response format for the state information query.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StateInfoResponse {
    pub manager: Addr,
    pub blob_count: u64,
}

// The response format for the state information query.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BlobInfoResponse {
    pub blob_id: u64,
    pub terra_block_number: u64,
    pub terra_previous_block: u64,
    pub data: Vec<String>, // Array of base64-encoded-data Strings
}
