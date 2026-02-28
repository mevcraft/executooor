use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::SolCall;

use crate::context::encode_context;
use crate::types::CallbackContext;
use crate::{callWithPlaceholders4845164670Call, call_g0oyU7oCall, exec_606BaXtCall, Placeholder};

/// The encoded transaction data ready to be sent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedExec {
    pub to: Address,
    pub data: Bytes,
    pub value: U256,
}

/// Builder for encoding batched calls to the Executor contract.
///
/// # Example
/// ```ignore
/// let mut encoder = ExecutorEncoder::new(executor_address);
/// encoder
///     .erc20_approve(dai, aave_pool, amount)
///     .aave_supply(aave_pool, dai, amount, None);
/// let tx = encoder.encode_exec(U256::ZERO);
/// ```
#[derive(Debug)]
pub struct ExecutorEncoder {
    address: Address,
    calls: Vec<Bytes>,
    total_value: U256,
}

impl ExecutorEncoder {
    pub fn new(address: Address) -> Self {
        Self {
            address,
            calls: Vec::new(),
            total_value: U256::ZERO,
        }
    }

    /// Returns the executor contract address.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Encodes a single call instruction (static/associated function).
    ///
    /// If `placeholders` is non-empty, encodes as `callWithPlaceholders4845164670`.
    /// Otherwise, encodes as `call_g0oyU7o`.
    pub fn build_call(
        target: Address,
        value: U256,
        call_data: Bytes,
        context: Option<&CallbackContext>,
        placeholders: Vec<Placeholder>,
    ) -> Bytes {
        let default_ctx = CallbackContext::default();
        let ctx = context.unwrap_or(&default_ctx);
        let encoded_context = encode_context(ctx.sender, ctx.data_index);

        if !placeholders.is_empty() {
            callWithPlaceholders4845164670Call {
                target,
                value,
                context: encoded_context,
                callData: call_data,
                placeholders,
            }
            .abi_encode()
            .into()
        } else {
            call_g0oyU7oCall {
                target,
                value,
                context: encoded_context,
                callData: call_data,
            }
            .abi_encode()
            .into()
        }
    }

    /// Pushes an encoded call onto the internal call list.
    /// Returns `&mut Self` for method chaining.
    pub fn push_call(
        &mut self,
        target: Address,
        value: U256,
        call_data: Bytes,
        context: Option<&CallbackContext>,
        placeholders: Vec<Placeholder>,
    ) -> &mut Self {
        self.total_value += value;
        self.calls.push(Self::build_call(
            target,
            value,
            call_data,
            context,
            placeholders,
        ));
        self
    }

    /// Drains and returns all accumulated calls, resetting internal state.
    pub fn flush(&mut self) -> Vec<Bytes> {
        self.total_value = U256::ZERO;
        std::mem::take(&mut self.calls)
    }

    /// Transfers ETH to the recipient via the Executor contract.
    ///
    /// # Panics
    ///
    /// Panics if `recipient` is `Address::ZERO` — use [`tip`](Self::tip) instead.
    pub fn transfer(&mut self, recipient: Address, amount: U256) -> &mut Self {
        assert!(
            recipient != Address::ZERO,
            "recipient should not be zero: use tip() instead"
        );
        let call_data = crate::executor_sol::transferCall { recipient, amount }
            .abi_encode()
            .into();
        self.push_call(self.address(), U256::ZERO, call_data, None, vec![])
    }

    /// Sends ETH to `block.coinbase` (miner/validator tip).
    pub fn tip(&mut self, amount: U256) -> &mut Self {
        let call_data = crate::executor_sol::transferCall {
            recipient: Address::ZERO,
            amount,
        }
        .abi_encode()
        .into();
        self.push_call(self.address(), U256::ZERO, call_data, None, vec![])
    }

    /// Encodes the full `exec_606BaXt(bytes[])` transaction.
    ///
    /// Consumes all accumulated calls (equivalent to calling [`flush`](Self::flush)).
    /// The encoder is reset and ready for the next batch after this call.
    pub fn encode_exec(&mut self, extra_value: U256) -> EncodedExec {
        let value = self.total_value + extra_value;
        let calls = self.flush();
        let data: Bytes = exec_606BaXtCall { data: calls }.abi_encode().into();
        EncodedExec {
            to: self.address,
            data,
            value,
        }
    }
}

/// Encodes callback data as `abi.encode(bytes[], bytes)`.
///
/// This is the standard pattern for all flash loan callbacks:
/// the first element is an array of calls to execute inside the callback,
/// the second element is a return value (often `0x` or a hash).
pub fn encode_callback_data(calls: Vec<Bytes>, return_value: Bytes) -> Bytes {
    use alloy_sol_types::SolValue;
    (calls, return_value).abi_encode_params().into()
}
