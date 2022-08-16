//! Module define supported protocols types.

use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub enum ProtocolType {
    Tulip,
    Francium,
}
