use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::SolCall;

use crate::encoder::ExecutorEncoder;
use crate::Placeholder;

pub(crate) mod erc20_sol {
    use alloy_sol_types::sol;

    sol! {
        function balanceOf(address owner) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function transfer(address to, uint256 amount) external returns (bool);
        function transferFrom(address from, address to, uint256 amount) external returns (bool);
    }
}

impl ExecutorEncoder {
    /// Builds an ERC20 approve call wrapped in the executor call (static method).
    ///
    /// Used internally by flash loan methods to construct repayment calls.
    pub fn build_erc20_approve(asset: Address, spender: Address, amount: U256) -> Bytes {
        let call_data: Bytes = erc20_sol::approveCall { spender, amount }
            .abi_encode()
            .into();
        Self::build_call(asset, U256::ZERO, call_data, None, vec![])
    }

    /// Builds an ERC20 transfer call wrapped in the executor call (static method).
    ///
    /// Used internally by flash loan methods to construct repayment calls.
    pub fn build_erc20_transfer(asset: Address, recipient: Address, amount: U256) -> Bytes {
        let call_data: Bytes = erc20_sol::transferCall {
            to: recipient,
            amount,
        }
        .abi_encode()
        .into();
        Self::build_call(asset, U256::ZERO, call_data, None, vec![])
    }

    /// Creates a `Placeholder` that reads `balanceOf(owner)` from `asset`.
    ///
    /// The result (32 bytes at response offset 0) is placed at `offset` in the call data.
    /// This is used by methods like `erc20_approve_all` and `erc20_skim`.
    pub fn erc20_balance_of(&self, asset: Address, owner: Address, offset: u64) -> Placeholder {
        Placeholder {
            to: asset,
            data: erc20_sol::balanceOfCall { owner }.abi_encode().into(),
            offset,
            length: 32,
            resOffset: 0,
        }
    }

    /// Approves `spender` to spend `allowance` of `asset`.
    pub fn erc20_approve(
        &mut self,
        asset: Address,
        spender: Address,
        allowance: U256,
    ) -> &mut Self {
        let call_data: Bytes = erc20_sol::approveCall {
            spender,
            amount: allowance,
        }
        .abi_encode()
        .into();
        self.push_call(asset, U256::ZERO, call_data, None, vec![])
    }

    /// Approves `spender` to spend the entire balance of `asset` held by the executor.
    ///
    /// Uses a placeholder to dynamically read the balance at execution time.
    pub fn erc20_approve_all(&mut self, asset: Address, spender: Address) -> &mut Self {
        let placeholder = self.erc20_balance_of(asset, self.address(), 4 + 32);
        let call_data: Bytes = erc20_sol::approveCall {
            spender,
            amount: U256::ZERO,
        }
        .abi_encode()
        .into();
        self.push_call(asset, U256::ZERO, call_data, None, vec![placeholder])
    }

    /// Transfers `amount` of `asset` to `recipient`.
    pub fn erc20_transfer(
        &mut self,
        asset: Address,
        recipient: Address,
        amount: U256,
    ) -> &mut Self {
        let call_data: Bytes = erc20_sol::transferCall {
            to: recipient,
            amount,
        }
        .abi_encode()
        .into();
        self.push_call(asset, U256::ZERO, call_data, None, vec![])
    }

    /// Transfers `amount` of `asset` from `owner` to `recipient`.
    pub fn erc20_transfer_from(
        &mut self,
        asset: Address,
        owner: Address,
        recipient: Address,
        amount: U256,
    ) -> &mut Self {
        let call_data: Bytes = erc20_sol::transferFromCall {
            from: owner,
            to: recipient,
            amount,
        }
        .abi_encode()
        .into();
        self.push_call(asset, U256::ZERO, call_data, None, vec![])
    }

    /// Transfers the entire balance of `asset` to `recipient` (skim).
    ///
    /// Uses a placeholder to dynamically read the balance at execution time.
    pub fn erc20_skim(&mut self, asset: Address, recipient: Address) -> &mut Self {
        let placeholder = self.erc20_balance_of(asset, self.address(), 4 + 32);
        let call_data: Bytes = erc20_sol::transferCall {
            to: recipient,
            amount: U256::ZERO,
        }
        .abi_encode()
        .into();
        self.push_call(asset, U256::ZERO, call_data, None, vec![placeholder])
    }
}
