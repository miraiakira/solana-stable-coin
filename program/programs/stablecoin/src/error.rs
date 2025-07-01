use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvalidPrice,

    #[msg("Health factor below minimum")]
    BelowMinHealthFactor,
}