use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::{sol, SolCall};

use crate::encoder::ExecutorEncoder;

sol! {
    struct ExactInputParams {
        bytes path;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
    }

    function exactInput(ExactInputParams params) external payable returns (uint256 amountOut);

    struct ExactOutputParams {
        bytes path;
        address recipient;
        uint256 deadline;
        uint256 amountOut;
        uint256 amountInMaximum;
    }

    function exactOutput(ExactOutputParams params) external payable returns (uint256 amountIn);
}

impl ExecutorEncoder {
    /// Swaps using UniswapV3 `exactInput`.
    pub fn uni_v3_exact_input(
        &mut self,
        router: Address,
        path: Bytes,
        amount_in: U256,
        amount_out_minimum: U256,
        deadline: U256,
        recipient: Option<Address>,
    ) -> &mut Self {
        let recipient = recipient.unwrap_or(self.address());
        let call_data = exactInputCall {
            params: ExactInputParams {
                path,
                recipient,
                deadline,
                amountIn: amount_in,
                amountOutMinimum: amount_out_minimum,
            },
        }
        .abi_encode()
        .into();
        self.push_call(router, U256::ZERO, call_data, None, vec![])
    }

    /// Swaps using UniswapV3 `exactInput` with the entire balance of the input token.
    ///
    /// Uses a placeholder to dynamically read the balance at execution time.
    /// The first 20 bytes of `path` are the input token address.
    ///
    /// # Panics
    ///
    /// Panics if `path` is shorter than 20 bytes.
    pub fn uni_v3_exact_input_all(
        &mut self,
        router: Address,
        path: Bytes,
        amount_out_minimum: U256,
        deadline: U256,
        recipient: Option<Address>,
    ) -> &mut Self {
        assert!(path.len() >= 20, "path must be at least 20 bytes");
        let recipient = recipient.unwrap_or(self.address());
        // Extract input token from the first 20 bytes of the path
        let input_token = Address::from_slice(&path[..20]);
        let placeholder = self.erc20_balance_of(input_token, self.address(), 4 + 32 * 4);
        let call_data = exactInputCall {
            params: ExactInputParams {
                path,
                recipient,
                deadline,
                amountIn: U256::ZERO,
                amountOutMinimum: amount_out_minimum,
            },
        }
        .abi_encode()
        .into();
        self.push_call(router, U256::ZERO, call_data, None, vec![placeholder])
    }

    /// Swaps using UniswapV3 `exactOutput`.
    pub fn uni_v3_exact_output(
        &mut self,
        router: Address,
        path: Bytes,
        amount_out: U256,
        amount_in_maximum: U256,
        deadline: U256,
        recipient: Option<Address>,
    ) -> &mut Self {
        let recipient = recipient.unwrap_or(self.address());
        let call_data = exactOutputCall {
            params: ExactOutputParams {
                path,
                recipient,
                deadline,
                amountOut: amount_out,
                amountInMaximum: amount_in_maximum,
            },
        }
        .abi_encode()
        .into();
        self.push_call(router, U256::ZERO, call_data, None, vec![])
    }
}
