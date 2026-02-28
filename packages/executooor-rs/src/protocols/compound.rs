use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;

use crate::encoder::ExecutorEncoder;

mod compound_sol {
    use alloy_sol_types::sol;

    sol! {
        function mint(uint256 amount) external;
        function borrow(uint256 amount) external;
        function repayBorrow(uint256 amount) external;
        function repayBorrowBehalf(address onBehalfOf, uint256 amount) external;
        function redeemUnderlying(uint256 amount) external;
    }
}

impl ExecutorEncoder {
    /// Supplies `amount` to the Compound cToken (calls `mint`).
    pub fn compound_supply(&mut self, c_token: Address, amount: U256) -> &mut Self {
        let call_data = compound_sol::mintCall { amount }.abi_encode().into();
        self.push_call(c_token, U256::ZERO, call_data, None, vec![])
    }

    /// Borrows `amount` from the Compound cToken.
    pub fn compound_borrow(&mut self, c_token: Address, amount: U256) -> &mut Self {
        let call_data = compound_sol::borrowCall { amount }.abi_encode().into();
        self.push_call(c_token, U256::ZERO, call_data, None, vec![])
    }

    /// Repays `amount` to the Compound cToken.
    ///
    /// If `on_behalf_of` is provided, calls `repayBorrowBehalf`.
    pub fn compound_repay(
        &mut self,
        c_token: Address,
        amount: U256,
        on_behalf_of: Option<Address>,
    ) -> &mut Self {
        if let Some(beneficiary) = on_behalf_of {
            let call_data = compound_sol::repayBorrowBehalfCall {
                onBehalfOf: beneficiary,
                amount,
            }
            .abi_encode()
            .into();
            self.push_call(c_token, U256::ZERO, call_data, None, vec![])
        } else {
            let call_data = compound_sol::repayBorrowCall { amount }.abi_encode().into();
            self.push_call(c_token, U256::ZERO, call_data, None, vec![])
        }
    }

    /// Withdraws `amount` from the Compound cToken (calls `redeemUnderlying`).
    pub fn compound_withdraw(&mut self, c_token: Address, amount: U256) -> &mut Self {
        let call_data = compound_sol::redeemUnderlyingCall { amount }
            .abi_encode()
            .into();
        self.push_call(c_token, U256::ZERO, call_data, None, vec![])
    }
}
