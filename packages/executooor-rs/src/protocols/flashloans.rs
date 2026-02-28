use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::SolCall;

use crate::encoder::{encode_callback_data, ExecutorEncoder};
use crate::math::{mul_div_up, percent_mul};
use crate::types::{AssetRequest, CallbackContext};

mod balancer_sol {
    use alloy_sol_types::sol;

    sol! {
        function flashLoan(
            address recipient,
            address[] tokens,
            uint256[] amounts,
            bytes userData
        ) external;
    }
}

mod maker_sol {
    use alloy_sol_types::sol;

    sol! {
        function flashLoan(
            address receiver,
            address token,
            uint256 amount,
            bytes data
        ) external;
    }
}

mod aave_pool_sol {
    use alloy_sol_types::sol;

    sol! {
        function flashLoan(
            address receiverAddress,
            address[] assets,
            uint256[] amounts,
            uint256[] modes,
            address onBehalfOf,
            bytes params,
            uint16 referralCode
        ) external;
    }
}

mod uni_flash_sol {
    use alloy_sol_types::sol;

    sol! {
        function flash(
            address receiver,
            uint256 amount0,
            uint256 amount1,
            bytes data
        ) external;
    }
}

mod morpho_blue_flash_sol {
    use alloy_sol_types::sol;

    sol! {
        function flashLoan(
            address asset,
            uint256 amount,
            bytes data
        ) external;
    }
}

impl ExecutorEncoder {
    /// Executes a Balancer flash loan.
    ///
    /// Callback data index = 3 (receiveFlashLoan has `bytes` at arg index 3).
    /// Automatically appends transfer repayment calls.
    pub fn balancer_flash_loan(
        &mut self,
        vault: Address,
        requests: &[AssetRequest],
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();

        let repayment_calls: Vec<Bytes> = requests
            .iter()
            .map(|r| Self::build_erc20_transfer(r.asset, vault, r.amount))
            .collect();

        let mut all_calls = callback_calls;
        all_calls.extend(repayment_calls);

        let user_data = encode_callback_data(all_calls, Bytes::new());

        let tokens: Vec<Address> = requests.iter().map(|r| r.asset).collect();
        let amounts: Vec<U256> = requests.iter().map(|r| r.amount).collect();

        let call_data = balancer_sol::flashLoanCall {
            recipient: self.address(),
            tokens,
            amounts,
            userData: user_data,
        }
        .abi_encode()
        .into();

        let ctx = CallbackContext {
            sender: vault,
            data_index: 3,
        };
        self.push_call(vault, U256::ZERO, call_data, Some(&ctx), vec![])
    }

    /// Executes a Maker (ERC3156) flash loan.
    ///
    /// Callback data index = 4 (onFlashLoan has `bytes` at arg index 4).
    /// Returns `keccak256("ERC3156FlashBorrower.onFlashLoan")` as the callback return value.
    pub fn maker_flash_loan(
        &mut self,
        vault: Address,
        asset: Address,
        amount: U256,
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();

        let mut all_calls = callback_calls;
        all_calls.push(Self::build_erc20_approve(asset, vault, amount));

        let return_value = Bytes::from(
            alloy_primitives::keccak256("ERC3156FlashBorrower.onFlashLoan")
                .as_slice()
                .to_vec(),
        );
        let data = encode_callback_data(all_calls, return_value);

        let call_data = maker_sol::flashLoanCall {
            receiver: self.address(),
            token: asset,
            amount,
            data,
        }
        .abi_encode()
        .into();

        let ctx = CallbackContext {
            sender: vault,
            data_index: 4,
        };
        self.push_call(vault, U256::ZERO, call_data, Some(&ctx), vec![])
    }

    /// Executes an Aave flash loan.
    ///
    /// Callback data index = 4 (executeOperation has `bytes` at arg index 4).
    /// `premium` is the Aave flash loan fee in basis points (e.g., 5 = 0.05%).
    pub fn aave_flash_loan(
        &mut self,
        pool: Address,
        requests: &[AssetRequest],
        premium: U256,
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();

        let approval_calls: Vec<Bytes> = requests
            .iter()
            .map(|r| {
                let approval_amount = r.amount + percent_mul(r.amount, premium);
                Self::build_erc20_approve(r.asset, pool, approval_amount)
            })
            .collect();

        let mut all_calls = callback_calls;
        all_calls.extend(approval_calls);

        // Return value: 0x0000...0001 (true, indicating successful execution)
        let return_value = Bytes::from({
            let mut b = vec![0u8; 32];
            b[31] = 1;
            b
        });
        let params = encode_callback_data(all_calls, return_value);

        let assets: Vec<Address> = requests.iter().map(|r| r.asset).collect();
        let amounts: Vec<U256> = requests.iter().map(|r| r.amount).collect();
        let modes: Vec<U256> = requests.iter().map(|_| U256::ZERO).collect();

        let call_data = aave_pool_sol::flashLoanCall {
            receiverAddress: self.address(),
            assets,
            amounts,
            modes,
            onBehalfOf: self.address(),
            params,
            referralCode: 0,
        }
        .abi_encode()
        .into();

        let ctx = CallbackContext {
            sender: pool,
            data_index: 4,
        };
        self.push_call(pool, U256::ZERO, call_data, Some(&ctx), vec![])
    }

    /// Executes a Uniswap V2 flash swap.
    ///
    /// Callback data index = 3 (uniswapV2Call has `bytes` at arg index 3).
    ///
    /// **Warning:** This method currently uses the UniV3 `flash` function signature
    /// instead of the V2 pair's `swap`. Fee calculation is also unimplemented (hardcoded to 0).
    /// This matches the upstream TS implementation which has the same TODO.
    /// Do not use on real V2 pools without fixing these issues.
    pub fn uni_v2_flash_swap(
        &mut self,
        pool: Address,
        assets: [Address; 2],
        amounts: [U256; 2],
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();

        // TODO: calculate fee
        let fee0 = U256::ZERO;
        let fee1 = U256::ZERO;

        let mut all_calls = callback_calls;
        all_calls.push(Self::build_erc20_approve(
            assets[0],
            pool,
            amounts[0] + fee0,
        ));
        all_calls.push(Self::build_erc20_approve(
            assets[1],
            pool,
            amounts[1] + fee1,
        ));

        let data = encode_callback_data(all_calls, Bytes::new());

        let call_data = uni_flash_sol::flashCall {
            receiver: self.address(),
            amount0: amounts[0],
            amount1: amounts[1],
            data,
        }
        .abi_encode()
        .into();

        let ctx = CallbackContext {
            sender: pool,
            data_index: 3,
        };
        self.push_call(pool, U256::ZERO, call_data, Some(&ctx), vec![])
    }

    /// Executes a Uniswap V3 flash loan.
    ///
    /// Callback data index = 2 (uniswapV3FlashCallback has `bytes` at arg index 2).
    /// `fee` is in basis points (e.g., 500 = 0.05%).
    pub fn uni_v3_flash_loan(
        &mut self,
        pool: Address,
        assets: [Address; 2],
        amounts: [U256; 2],
        fee: U256,
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();

        let fee0 = mul_div_up(amounts[0], fee, U256::from(1_000_000u64));
        let fee1 = mul_div_up(amounts[1], fee, U256::from(1_000_000u64));

        let mut all_calls = callback_calls;
        all_calls.push(Self::build_erc20_transfer(
            assets[0],
            pool,
            amounts[0] + fee0,
        ));
        all_calls.push(Self::build_erc20_transfer(
            assets[1],
            pool,
            amounts[1] + fee1,
        ));

        let data = encode_callback_data(all_calls, Bytes::new());

        let call_data = uni_flash_sol::flashCall {
            receiver: self.address(),
            amount0: amounts[0],
            amount1: amounts[1],
            data,
        }
        .abi_encode()
        .into();

        let ctx = CallbackContext {
            sender: pool,
            data_index: 2,
        };
        self.push_call(pool, U256::ZERO, call_data, Some(&ctx), vec![])
    }

    /// Executes a Morpho Blue flash loan.
    ///
    /// Callback data index = 1 (onMorphoFlashLoan has `bytes` at arg index 1).
    pub fn blue_flash_loan(
        &mut self,
        morpho_blue: Address,
        asset: Address,
        amount: U256,
        callback_calls: Option<Vec<Bytes>>,
    ) -> &mut Self {
        let callback_calls = callback_calls.unwrap_or_default();

        let mut all_calls = callback_calls;
        all_calls.push(Self::build_erc20_approve(asset, morpho_blue, amount));

        let data = encode_callback_data(all_calls, Bytes::new());

        let call_data = morpho_blue_flash_sol::flashLoanCall {
            asset,
            amount,
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
