use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Too many poll options")]
    TooManyOptions {},

    #[error("Poll not found")]
    PollNotFound {},

    #[error("Option not found")]
    OptionNotFound {},

    #[error("Poll is closed")]
    PollIsClosed {},
}
