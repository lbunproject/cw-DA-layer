use cosmwasm_std::{ StdError };
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")] Std(#[from] StdError),

    #[error("Unauthorized")] Unauthorized {},

    #[error("{err_message}")] GenericError { err_message: String, },

    #[error("Debug Point at {location}")] TracePoint { location: String, },
}
