use cosmwasm_std::{ entry_point,  to_binary, Binary, Deps,  DepsMut, Env, MessageInfo, Response, StdResult };
use crate::error::ContractError;
use crate::msg::{ MigrateMsg, InstantiateMsg, ExecuteMsg, QueryMsg };
use crate::state::{ CONTRACT_STATE, ContractState, BLOB_DATA, BlobData };
use crate::execute::{ try_change_manager, try_submit_blob };
use crate::query::{ get_state_info, get_blob_by_id, get_blob_by_block };
use crate::helper::{ check_authorized_or_manager, check_authorized_only };

//Authorized addresses
pub const AUTHORIZED_ADDRESS: &str = "terra12pvn7k6uc7hptqsdegg2synz4zgetqplr7ls8u";
pub const MANAGER_ADDRESS: &str = "terra1vgdez00kxrdnj5xkzey3nvzxj8rjgw0rspxgpw";

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Migrate (Update contract code)
////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Instantiate
////////////////////////////////////////////////////////////////////////////////////////////////////////
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    // Check Permissions
    //check_authorized_only(deps.storage, info.sender.clone())?;
    
    // Initialize contract state
    let state = ContractState {
        manager: info.sender.clone(),
        blob_count: 0,
    };
    CONTRACT_STATE.save(deps.storage, &state)?;

    // Initialize the BlobData structure
    let blob_data = BlobData { blobs: vec![] };
    BLOB_DATA.save(deps.storage, &blob_data)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Execute
////////////////////////////////////////////////////////////////////////////////////////////////////////
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ChangeManager { new_manager } => try_change_manager(deps, info, new_manager),
        ExecuteMsg::SubmitBlob { contents } => try_submit_blob(deps, env, info, contents),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Queries
////////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStateInfo {} => to_binary(&get_state_info(deps)?),
        QueryMsg::GetBlobById { blob_id } => to_binary(&get_blob_by_id(deps, blob_id)?),
        QueryMsg::GetBlobByBlock { terra_block_number } => to_binary(&get_blob_by_block(deps, terra_block_number)?),
    }
}
