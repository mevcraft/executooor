use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::SolCall;

use crate::encoder::{encode_callback_data, ExecutorEncoder};
use crate::types::{CallbackContext, MarketParams};

mod morpho_blue_sol {
    use alloy_sol_types::sol;

    sol! {
        struct MarketParams {
            address loanToken;
            address collateralToken;
            address oracle;
            address irm;
            uint256 lltv;
        }

        function supplyCollateral(
            MarketParams marketParams,
            uint256 assets,
            address onBehalf,
            bytes data
        ) external;

        function withdrawCollateral(
            MarketParams marketParams,
            uint256 assets,
            address onBehalf,
            address receiver
        ) external;

        function supply(
            MarketParams marketParams,
            uint256 assets,
            uint256 shares,
            address onBehalf,
            bytes data
        ) external;

        function withdraw(
            MarketParams marketParams,
            uint256 assets,
            uint256 shares,
            address onBehalf,
            address receiver
        ) external;

        function repay(
            MarketParams marketParams,
            uint256 assets,
            uint256 shares,
            address onBehalf,
            bytes data
        ) external;

        function borrow(
            MarketParams marketParams,
            uint256 assets,
            uint256 shares,
            address onBehalf,
            address receiver
        ) external;

        function liquidate(
            MarketParams marketParams,
            address borrower,
            uint256 seizedAssets,
            uint256 repaidShares,
            bytes data
        ) external;
    }
}

fn to_sol_market(m: &MarketParams) -> morpho_blue_sol::MarketParams {
    morpho_blue_sol::MarketParams {
        loanToken: m.loan_token,
        collateralToken: m.collateral_token,
        oracle: m.oracle,
        irm: m.irm,
        lltv: m.lltv,
    }
}

impl ExecutorEncoder {
    /// Supplies collateral to Morpho Blue.
    ///
    /// Callback data index = 1 (onMorphoSupplyCollateral).
    pub fn morpho_blue_supply_collateral(
        &mut self,
        morpho_blue: Address,
        market: &MarketParams,
        collateral: U256,
        on_behalf: Address,
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();
        let data = encode_callback_data(callback_calls, Bytes::new());

        let call_data = morpho_blue_sol::supplyCollateralCall {
            marketParams: to_sol_market(market),
            assets: collateral,
            onBehalf: on_behalf,
            data,
        }
        .abi_encode()
        .into();

        let ctx = CallbackContext {
            sender: morpho_blue,
            data_index: 1,
        };
        self.push_call(morpho_blue, U256::ZERO, call_data, Some(&ctx), vec![])
    }

    /// Withdraws collateral from Morpho Blue.
    pub fn morpho_blue_withdraw_collateral(
        &mut self,
        morpho_blue: Address,
        market: &MarketParams,
        collateral: U256,
        on_behalf: Address,
        receiver: Address,
    ) -> &mut Self {
        let call_data = morpho_blue_sol::withdrawCollateralCall {
            marketParams: to_sol_market(market),
            assets: collateral,
            onBehalf: on_behalf,
            receiver,
        }
        .abi_encode()
        .into();
        self.push_call(morpho_blue, U256::ZERO, call_data, None, vec![])
    }

    /// Supplies to Morpho Blue.
    ///
    /// Callback data index = 1 (onMorphoSupply).
    pub fn morpho_blue_supply(
        &mut self,
        morpho_blue: Address,
        market: &MarketParams,
        assets: U256,
        shares: U256,
        on_behalf: Address,
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();
        let data = encode_callback_data(callback_calls, Bytes::new());

        let call_data = morpho_blue_sol::supplyCall {
            marketParams: to_sol_market(market),
            assets,
            shares,
            onBehalf: on_behalf,
            data,
        }
        .abi_encode()
        .into();

        let ctx = CallbackContext {
            sender: morpho_blue,
            data_index: 1,
        };
        self.push_call(morpho_blue, U256::ZERO, call_data, Some(&ctx), vec![])
    }

    /// Withdraws from Morpho Blue.
    pub fn morpho_blue_withdraw(
        &mut self,
        morpho_blue: Address,
        market: &MarketParams,
        assets: U256,
        shares: U256,
        on_behalf: Address,
        receiver: Address,
    ) -> &mut Self {
        let call_data = morpho_blue_sol::withdrawCall {
            marketParams: to_sol_market(market),
            assets,
            shares,
            onBehalf: on_behalf,
            receiver,
        }
        .abi_encode()
        .into();
        self.push_call(morpho_blue, U256::ZERO, call_data, None, vec![])
    }

    /// Repays to Morpho Blue.
    ///
    /// Callback data index = 1 (onMorphoRepay).
    pub fn morpho_blue_repay(
        &mut self,
        morpho_blue: Address,
        market: &MarketParams,
        assets: U256,
        shares: U256,
        on_behalf: Address,
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();
        let data = encode_callback_data(callback_calls, Bytes::new());

        let call_data = morpho_blue_sol::repayCall {
            marketParams: to_sol_market(market),
            assets,
            shares,
            onBehalf: on_behalf,
            data,
        }
        .abi_encode()
        .into();

        let ctx = CallbackContext {
            sender: morpho_blue,
            data_index: 1,
        };
        self.push_call(morpho_blue, U256::ZERO, call_data, Some(&ctx), vec![])
    }

    /// Borrows from Morpho Blue.
    pub fn morpho_blue_borrow(
        &mut self,
        morpho_blue: Address,
        market: &MarketParams,
        assets: U256,
        shares: U256,
        on_behalf: Address,
        receiver: Address,
    ) -> &mut Self {
        let call_data = morpho_blue_sol::borrowCall {
            marketParams: to_sol_market(market),
            assets,
            shares,
            onBehalf: on_behalf,
            receiver,
        }
        .abi_encode()
        .into();
        self.push_call(morpho_blue, U256::ZERO, call_data, None, vec![])
    }

    /// Liquidates a position on Morpho Blue.
    ///
    /// Callback data index = 1 (onMorphoLiquidate).
    pub fn morpho_blue_liquidate(
        &mut self,
        morpho_blue: Address,
        market: &MarketParams,
        borrower: Address,
        seized_assets: U256,
        repaid_shares: U256,
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();
        let data = encode_callback_data(callback_calls, Bytes::new());

        let call_data = morpho_blue_sol::liquidateCall {
            marketParams: to_sol_market(market),
            borrower,
            seizedAssets: seized_assets,
            repaidShares: repaid_shares,
            data,
        }
        .abi_encode()
        .into();

        let ctx = CallbackContext {
            sender: morpho_blue,
            data_index: 1,
        };
        self.push_call(morpho_blue, U256::ZERO, call_data, Some(&ctx), vec![])
    }
}
