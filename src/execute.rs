use cosmwasm_std::{ DepsMut, Env, MessageInfo, Response};
use crate::error::{ ContractError };
use crate::state::{ CONTRACT_STATE, BLOB_DATA, Blob};
use crate::helper::{ check_authorized_or_manager };
use crate::msg::{ ContentsMsg };

//---------------------------------------------------------------------------------------------------------
// Change Manager
//---------------------------------------------------------------------------------------------------------
pub fn try_change_manager(
    deps: DepsMut,
    info: MessageInfo,
    new_manager: String
) -> Result<Response, ContractError> {
    
    // Check Permissions
    check_authorized_or_manager(deps.storage, info.sender)?;

    // Validate the new manager address
    let new_manager_addr = deps.api.addr_validate(&new_manager)?;

    // Update settings
    let mut state = CONTRACT_STATE.load(deps.storage)?;
    state.manager = new_manager_addr;
    CONTRACT_STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

//---------------------------------------------------------------------------------------------------------
// Submit Blob
//---------------------------------------------------------------------------------------------------------
pub fn try_submit_blob(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contents: ContentsMsg,
) -> Result<Response, ContractError> {
    // Check Permissions
    //check_authorized_or_manager(deps.storage, info.sender)?;

    // Check if the request type is "submit"
    if contents.action != "submit" {
        return Err(ContractError::GenericError {
            err_message: format!("Error: {}", "Invalid Request Action".to_string())
        });
    }

    // Load current state
    let mut state = CONTRACT_STATE.load(deps.storage)?;

    // Load the BlobData
    let mut blob_data = BLOB_DATA.load(deps.storage)?;

    // Determine the previous block number
    let previous_block_number = if contents.blob_id > 1 {
        if let Some(prev_blob) = blob_data.blobs.iter().find(|&blob| blob.blob_id == contents.blob_id - 1) {
            prev_blob.terra_block_number
        } else {
            return Err(ContractError::GenericError {
                err_message: format!("Error: {}", "Previous Blob not found".to_string())
            });
        }
    } else {
        0 // Fallback for the first blob
    };

    // Create a new Blob
    let new_blob = Blob {
        blob_id: contents.blob_id,
        terra_block_number: env.block.height,
        terra_previous_block: previous_block_number,
        data: contents.message.clone(), // Use the Vec<String> directly
    };

    // Add the new Blob to BlobData
    blob_data.blobs.push(new_blob);

    // Increment the blob count
    state.blob_count += 1;

    // Save updated BlobData and state
    BLOB_DATA.save(deps.storage, &blob_data)?;
    CONTRACT_STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "submit_blob")
        .add_attribute("blob_id", contents.blob_id.to_string())
        .add_attribute("terra_block_number", env.block.height.to_string())
        .add_attribute("terra_previous_block", previous_block_number.to_string())
        .add_attribute("blob_count", state.blob_count.to_string()))
}
