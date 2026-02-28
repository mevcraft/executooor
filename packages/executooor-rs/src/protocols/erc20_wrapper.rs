use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;

use crate::encoder::ExecutorEncoder;

mod erc20_wrapper_sol {
    use alloy_sol_types::sol;

    sol! {
        function depositFor(address owner, uint256 amount) external;
        function withdrawTo(address receiver, uint256 amount) external;
    }
}

impl ExecutorEncoder {
    /// Deposits `amount` of the underlying token into the wrapper on behalf of `on_behalf`.
    pub fn erc20_wrapper_deposit_for(
        &mut self,
        asset: Address,
        on_behalf: Address,
        amount: U256,
    ) -> &mut Self {
        let call_data = erc20_wrapper_sol::depositForCall {
            owner: on_behalf,
            amount,
        }
        .abi_encode()
        .into();
        self.push_call(asset, U256::ZERO, call_data, None, vec![])
    }

    /// Deposits the entire balance of `underlying` into the wrapper on behalf of `on_behalf`.
    ///
    /// Uses a placeholder to dynamically read the underlying balance at execution time.
    pub fn erc20_wrapper_deposit_all_for(
        &mut self,
        asset: Address,
        underlying: Address,
        on_behalf: Address,
    ) -> &mut Self {
        let placeholder = self.erc20_balance_of(underlying, self.address(), 4 + 32);
        let call_data = erc20_wrapper_sol::depositForCall {
            owner: on_behalf,
            amount: U256::ZERO,
        }
        .abi_encode()
        .into();
        self.push_call(asset, U256::ZERO, call_data, None, vec![placeholder])
    }

    /// Withdraws `amount` from the wrapper to `receiver`.
    pub fn erc20_wrapper_withdraw_to(
        &mut self,
        asset: Address,
        receiver: Address,
        amount: U256,
    ) -> &mut Self {
        let call_data = erc20_wrapper_sol::withdrawToCall { receiver, amount }
            .abi_encode()
            .into();
        self.push_call(asset, U256::ZERO, call_data, None, vec![])
    }

    /// Withdraws the entire wrapper token balance to `receiver`.
    ///
    /// Uses a placeholder to dynamically read the wrapper balance at execution time.
    pub fn erc20_wrapper_withdraw_all_to(
        &mut self,
        asset: Address,
        receiver: Address,
    ) -> &mut Self {
        let placeholder = self.erc20_balance_of(asset, self.address(), 4 + 32);
        let call_data = erc20_wrapper_sol::withdrawToCall {
            receiver,
            amount: U256::ZERO,
        }
        .abi_encode()
        .into();
        self.push_call(asset, U256::ZERO, call_data, None, vec![placeholder])
    }
}
