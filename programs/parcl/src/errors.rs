use anchor_lang::prelude::*;

#[error_code]
pub enum LiquidationError {
    #[msg("The account does not meet margin requirements.")]
    InsufficientMargin,

    #[msg("Error processing liquidation.")]
    LiquidationFailed,

    #[msg("No positions found in the account.")]
    NoPositionsFound,

    #[msg("Failed to retrieve data.")]
    DataRetrievalFailed,

    #[msg("Invalid operation attempted.")]
    InvalidOperation,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Price is not available.")]
    PriceUnavailable,
    #[msg("Market ID is invalid.")]
    InvalidMarketId,
}
