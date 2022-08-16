use anchor_lang::prelude::*;

#[error_code]
pub enum ChamberError {
    /// 6000.
    #[msg("Unsupported protocol error.")]
    UnsupportedProtocol,

    /// 6001.
    #[msg("Math overflow error.")]
    MathOverflow,

    /// 6002.
    #[msg("Insufficient user position funds error.")]
    InsufficientUserPositionFunds,
}
