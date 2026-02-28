use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;

use crate::encoder::ExecutorEncoder;

mod erc4626_sol {
    use alloy_sol_types::sol;

    sol! {
        function deposit(uint256 assets, address receiver) external returns (uint256);
        function mint(uint256 shares, address receiver) external returns (uint256);
        function withdraw(uint256 assets, address receiver, address owner) external returns (uint256);
        function redeem(uint256 shares, address receiver, address owner) external returns (uint256);
    }
}

impl ExecutorEncoder {
    /// Deposits `assets` into the ERC4626 vault.
    ///
    /// `owner` is the receiver of the vault shares (named to match the TS API).
    pub fn erc4626_deposit(&mut self, vault: Address, assets: U256, owner: Address) -> &mut Self {
        let call_data = erc4626_sol::depositCall {
            assets,
            receiver: owner,
        }
        .abi_encode()
        .into();
        self.push_call(vault, U256::ZERO, call_data, None, vec![])
    }

    /// Deposits the entire balance of `asset` into the vault for `owner`.
    ///
    /// Uses a placeholder to dynamically read the balance at execution time.
    pub fn erc4626_deposit_all(
        &mut self,
        vault: Address,
        asset: Address,
        owner: Address,
    ) -> &mut Self {
        let placeholder = self.erc20_balance_of(asset, self.address(), 4);
        let call_data = erc4626_sol::depositCall {
            assets: U256::ZERO,
            receiver: owner,
        }
        .abi_encode()
        .into();
        self.push_call(vault, U256::ZERO, call_data, None, vec![placeholder])
    }

    /// Mints `shares` from the vault for `owner`.
    pub fn erc4626_mint(&mut self, vault: Address, shares: U256, owner: Address) -> &mut Self {
        let call_data = erc4626_sol::mintCall {
            shares,
            receiver: owner,
        }
        .abi_encode()
        .into();
        self.push_call(vault, U256::ZERO, call_data, None, vec![])
    }

    /// Withdraws `assets` from the vault.
    pub fn erc4626_withdraw(
        &mut self,
        vault: Address,
        assets: U256,
        receiver: Address,
        owner: Address,
    ) -> &mut Self {
        let call_data = erc4626_sol::withdrawCall {
            assets,
            receiver,
            owner,
        }
        .abi_encode()
        .into();
        self.push_call(vault, U256::ZERO, call_data, None, vec![])
    }

    /// Redeems `shares` from the vault.
    pub fn erc4626_redeem(
        &mut self,
        vault: Address,
        shares: U256,
        receiver: Address,
        owner: Address,
    ) -> &mut Self {
        let call_data = erc4626_sol::redeemCall {
            shares,
            receiver,
            owner,
        }
        .abi_encode()
        .into();
        self.push_call(vault, U256::ZERO, call_data, None, vec![])
    }

    /// Redeems the entire vault share balance.
    ///
    /// Uses a placeholder to dynamically read the share balance at execution time.
    pub fn erc4626_redeem_all(
        &mut self,
        vault: Address,
        receiver: Address,
        owner: Address,
    ) -> &mut Self {
        let placeholder = self.erc20_balance_of(vault, self.address(), 4);
        let call_data = erc4626_sol::redeemCall {
            shares: U256::ZERO,
            receiver,
            owner,
        }
        .abi_encode()
        .into();
        self.push_call(vault, U256::ZERO, call_data, None, vec![placeholder])
    }
}
