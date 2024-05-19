use cosmwasm_std::{ Addr, Storage};
use crate::error::{ ContractError };
use crate::state::{ CONTRACT_STATE };
use crate::contract::{AUTHORIZED_ADDRESS, MANAGER_ADDRESS};


//---------------------------------------------------------------------------------------------------------
// Permissions check (authorized only)
//--------------------------------------------------------------------------------------------------------
pub fn check_authorized_only(
    _store: &dyn Storage,
    address: Addr,
) -> Result<bool, ContractError> {
    if
        AUTHORIZED_ADDRESS.to_string() != address.to_string()
    {
        return Err(ContractError::Unauthorized {});
    }
    Ok(true)
}

//---------------------------------------------------------------------------------------------------------
// Permissions check (manager only)
//--------------------------------------------------------------------------------------------------------
pub fn check_manager_only(
    store: &dyn Storage,
    address: Addr,
) -> Result<bool, ContractError> {
    let config_state = CONTRACT_STATE.load(store)?;
    if
        config_state.manager != address.to_string() &&
        MANAGER_ADDRESS.to_string() != address.to_string()
    {
        return Err(ContractError::Unauthorized {});
    }
    Ok(true)
}

//---------------------------------------------------------------------------------------------------------
// Permissions check (authorized or manager)
//--------------------------------------------------------------------------------------------------------
pub fn check_authorized_or_manager(
    store: &dyn Storage,
    address: Addr,
) -> Result<bool, ContractError> {
    let config_state = CONTRACT_STATE.load(store)?;
    
    if AUTHORIZED_ADDRESS != address.as_str() &&
        config_state.manager != address &&
        MANAGER_ADDRESS != address.as_str()
    {
        return Err(ContractError::Unauthorized {});
    }
    Ok(true)
}