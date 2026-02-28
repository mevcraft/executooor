use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;

use crate::encoder::ExecutorEncoder;

mod weth_sol {
    use alloy_sol_types::sol;

    sol! {
        function deposit() external payable;
        function withdraw(uint256 wad) external;
    }
}

impl ExecutorEncoder {
    /// Wraps ETH into WETH by calling `deposit()` with `amount` as msg.value.
    pub fn wrap_eth(&mut self, weth: Address, amount: U256) -> &mut Self {
        let call_data = weth_sol::depositCall {}.abi_encode().into();
        self.push_call(weth, amount, call_data, None, vec![])
    }

    /// Unwraps WETH into ETH by calling `withdraw(amount)`.
    pub fn unwrap_eth(&mut self, weth: Address, amount: U256) -> &mut Self {
        let call_data = weth_sol::withdrawCall { wad: amount }.abi_encode().into();
        self.push_call(weth, U256::ZERO, call_data, None, vec![])
    }
}
