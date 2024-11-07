use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid denom for Staking")]
    InvalidDenomStaking { denom: String },

    #[error("Don't support multi denom")]
    UnsupportedMultiDenom {  },

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Address not whitelisted")]
    NotWhitelisted {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("To Do Error")]
    ToDo {},
}
