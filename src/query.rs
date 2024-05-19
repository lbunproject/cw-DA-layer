use cosmwasm_std::{Deps, StdResult, StdError, Binary, to_binary, Env, entry_point};
use crate::{
    msg::{ StateInfoResponse, BlobInfoResponse, QueryMsg },
    state::{ CONTRACT_STATE, BLOB_DATA },
};

pub fn get_state_info(deps: Deps) -> StdResult<StateInfoResponse> {
    let state = CONTRACT_STATE.load(deps.storage)?;
    Ok(StateInfoResponse {
        manager: state.manager,
        blob_count: state.blob_count,
    })
}

pub fn get_blob_by_id(deps: Deps, blob_id: u64) -> StdResult<BlobInfoResponse> {
    let blob_data = BLOB_DATA.load(deps.storage)?;
    
    // Find the blob with the given blob_id
    let blob = blob_data.blobs.iter()
        .find(|&b| b.blob_id == blob_id)
        .ok_or_else(|| StdError::not_found(&format!("Blob with ID {} not found", blob_id)))?;
    
    Ok(BlobInfoResponse {
        blob_id: blob.blob_id,
        terra_block_number: blob.terra_block_number,
        terra_previous_block: blob.terra_previous_block,
        data: blob.data.clone(),
    })
}

pub fn get_blob_by_block(deps: Deps, terra_block_number: u64) -> StdResult<BlobInfoResponse> {
    let blob_data = BLOB_DATA.load(deps.storage)?;
    
    // Find the blob with the given terra_block_number
    let blob = blob_data.blobs.iter()
        .find(|&b| b.terra_block_number == terra_block_number)
        .ok_or_else(|| StdError::not_found(&format!("Blob in Terra block number {} not found", terra_block_number)))?;
    
    Ok(BlobInfoResponse {
        blob_id: blob.blob_id,
        terra_block_number: blob.terra_block_number,
        terra_previous_block: blob.terra_previous_block,
        data: blob.data.clone(),
    })
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStateInfo {} => to_binary(&get_state_info(deps)?),
        QueryMsg::GetBlobById { blob_id } => to_binary(&get_blob_by_id(deps, blob_id)?),
        QueryMsg::GetBlobByBlock { terra_block_number } => to_binary(&get_blob_by_block(deps, terra_block_number)?),
    }
}
