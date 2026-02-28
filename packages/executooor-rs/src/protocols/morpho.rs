use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;

use crate::encoder::ExecutorEncoder;

mod morpho_sol {
    use alloy_sol_types::sol;

    sol! {
        function liquidate(
            address poolTokenBorrowed,
            address poolTokenCollateral,
            address borrower,
            uint256 amount
        ) external;
    }
}

impl ExecutorEncoder {
    /// Liquidates a position on Morpho Compound.
    pub fn morpho_compound_liquidate(
        &mut self,
        morpho_compound: Address,
        borrowed_pool_token: Address,
        collateral_pool_token: Address,
        borrower: Address,
        amount: U256,
    ) -> &mut Self {
        let call_data = morpho_sol::liquidateCall {
            poolTokenBorrowed: borrowed_pool_token,
            poolTokenCollateral: collateral_pool_token,
            borrower,
            amount,
        }
        .abi_encode()
        .into();
        self.push_call(morpho_compound, U256::ZERO, call_data, None, vec![])
    }

    /// Liquidates a position on Morpho Aave V2.
    pub fn morpho_aave_v2_liquidate(
        &mut self,
        morpho_aave_v2: Address,
        borrowed_pool_token: Address,
        collateral_pool_token: Address,
        borrower: Address,
        amount: U256,
    ) -> &mut Self {
        let call_data = morpho_sol::liquidateCall {
            poolTokenBorrowed: borrowed_pool_token,
            poolTokenCollateral: collateral_pool_token,
            borrower,
            amount,
        }
        .abi_encode()
        .into();
        self.push_call(morpho_aave_v2, U256::ZERO, call_data, None, vec![])
    }

    /// Liquidates a position on Morpho Aave V3.
    ///
    /// Same selector as the other liquidate functions: `liquidate(address,address,address,uint256)`.
    pub fn morpho_aave_v3_liquidate(
        &mut self,
        morpho_aave_v3: Address,
        underlying_borrowed: Address,
        underlying_collateral: Address,
        borrower: Address,
        amount: U256,
    ) -> &mut Self {
        let call_data = morpho_sol::liquidateCall {
            poolTokenBorrowed: underlying_borrowed,
            poolTokenCollateral: underlying_collateral,
            borrower,
            amount,
        }
        .abi_encode()
        .into();
        self.push_call(morpho_aave_v3, U256::ZERO, call_data, None, vec![])
    }
}
