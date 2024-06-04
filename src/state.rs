use schemars::JsonSchema;
use serde::{Deserialize, Serialize}; 
use cosmwasm_std::{ Addr };
use cw_storage_plus::Item;

//-------------------------------------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractState {
    pub manager: Addr,          
    pub blob_count: u64,
}

//-------------------------------------------------------------------

// Define the structure to store multiple blobs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlobData {
    pub blobs: Vec<Blob>,
}

// Define the structure to hold a single blob's data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Blob {
    pub blob_id: u64,
    pub terra_block_number: u64,
    pub terra_previous_block: u64,
    pub data: Vec<String>, // Array of base64-encoded-data Strings
}

//-------------------------------------------------------------------

pub const CONTRACT_STATE: Item<ContractState> = Item::new("contract_state");
pub const BLOB_DATA: Item<BlobData> = Item::new("blob_data");

//-------------------------------------------------------------------
