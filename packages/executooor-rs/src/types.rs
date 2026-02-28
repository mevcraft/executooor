use alloy_primitives::{Address, U256};

/// Context for callback-based calls.
///
/// `sender` is the address expected to call back.
/// `data_index` is the callback data parameter index in the callback function signature.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CallbackContext {
    pub sender: Address,
    pub data_index: u64,
}

/// A request for a specific amount of an asset (used in flash loans).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetRequest {
    pub asset: Address,
    pub amount: U256,
}

/// Morpho Blue market parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketParams {
    pub loan_token: Address,
    pub collateral_token: Address,
    pub oracle: Address,
    pub irm: Address,
    pub lltv: U256,
}
