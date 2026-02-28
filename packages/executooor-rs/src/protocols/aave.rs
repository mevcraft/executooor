use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;

use crate::encoder::ExecutorEncoder;

mod aave_sol {
    use alloy_sol_types::sol;

    sol! {
        function deposit(
            address asset,
            uint256 amount,
            address onBehalfOf,
            uint16 referralCode
        ) external;

        function borrow(
            address asset,
            uint256 amount,
            uint256 interestRateMode,
            uint16 referralCode,
            address onBehalfOf
        ) external;

        function repay(
            address asset,
            uint256 amount,
            uint256 interestRateMode,
            address onBehalfOf
        ) external;

        function withdraw(
            address asset,
            uint256 amount,
            address to
        ) external;

        function liquidationCall(
            address collateralAsset,
            address debtAsset,
            address user,
            uint256 debtToCover,
            bool receiveAToken
        ) external;
    }
}

impl ExecutorEncoder {
    /// Supplies `amount` of `asset` to the Aave pool.
    pub fn aave_supply(
        &mut self,
        pool: Address,
        asset: Address,
        amount: U256,
        on_behalf_of: Option<Address>,
    ) -> &mut Self {
        let on_behalf_of = on_behalf_of.unwrap_or(self.address());
        let call_data = aave_sol::depositCall {
            asset,
            amount,
            onBehalfOf: on_behalf_of,
            referralCode: 0,
        }
        .abi_encode()
        .into();
        self.push_call(pool, U256::ZERO, call_data, None, vec![])
    }

    /// Borrows `amount` of `asset` from the Aave pool.
    pub fn aave_borrow(
        &mut self,
        pool: Address,
        asset: Address,
        amount: U256,
        interest_rate_mode: U256,
        on_behalf_of: Option<Address>,
    ) -> &mut Self {
        let on_behalf_of = on_behalf_of.unwrap_or(self.address());
        let call_data = aave_sol::borrowCall {
            asset,
            amount,
            interestRateMode: interest_rate_mode,
            referralCode: 0,
            onBehalfOf: on_behalf_of,
        }
        .abi_encode()
        .into();
        self.push_call(pool, U256::ZERO, call_data, None, vec![])
    }

    /// Repays `amount` of `asset` to the Aave pool.
    pub fn aave_repay(
        &mut self,
        pool: Address,
        asset: Address,
        amount: U256,
        interest_rate_mode: U256,
        on_behalf_of: Option<Address>,
    ) -> &mut Self {
        let on_behalf_of = on_behalf_of.unwrap_or(self.address());
        let call_data = aave_sol::repayCall {
            asset,
            amount,
            interestRateMode: interest_rate_mode,
            onBehalfOf: on_behalf_of,
        }
        .abi_encode()
        .into();
        self.push_call(pool, U256::ZERO, call_data, None, vec![])
    }

    /// Withdraws `amount` of `asset` from the Aave pool.
    pub fn aave_withdraw(
        &mut self,
        pool: Address,
        asset: Address,
        amount: U256,
        to: Option<Address>,
    ) -> &mut Self {
        let to = to.unwrap_or(self.address());
        let call_data = aave_sol::withdrawCall { asset, amount, to }
            .abi_encode()
            .into();
        self.push_call(pool, U256::ZERO, call_data, None, vec![])
    }

    /// Liquidates a position on the Aave pool.
    pub fn aave_liquidate(
        &mut self,
        pool: Address,
        collateral: Address,
        debt: Address,
        user: Address,
        amount: U256,
    ) -> &mut Self {
        let call_data = aave_sol::liquidationCallCall {
            collateralAsset: collateral,
            debtAsset: debt,
            user,
            debtToCover: amount,
            receiveAToken: false,
        }
        .abi_encode()
        .into();
        self.push_call(pool, U256::ZERO, call_data, None, vec![])
    }
}
