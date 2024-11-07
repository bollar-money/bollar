use cosmwasm_std::StdError;
use cw20_base::ContractError as Cw20ContractError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Cw20Contract(#[from] Cw20ContractError),

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid expiration value")]
    InvalidExpiration {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Address not whitelisted")]
    NotWhitelisted {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("UnsupportDenom")]
    UnsupportDenom { denom: String },

    #[error("To Do Error")]
    ToDo {},
}
