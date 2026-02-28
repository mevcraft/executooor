use alloy_sol_types::sol;

pub mod context;
pub mod encoder;
pub mod math;
pub mod protocols;
pub mod types;

pub use context::encode_context;
pub use encoder::{EncodedExec, ExecutorEncoder};
pub use types::{AssetRequest, CallbackContext, MarketParams};

sol! {
    /// Placeholder struct for dynamic data injection via staticcalls.
    struct Placeholder {
        address to;
        bytes data;
        uint64 offset;
        uint64 length;
        uint64 resOffset;
    }

    /// Executes a batch of calls.
    function exec_606BaXt(bytes[] data);

    /// Executes a single call.
    function call_g0oyU7o(address target, uint256 value, bytes32 context, bytes callData);

    /// Executes a call with placeholder data injection.
    function callWithPlaceholders4845164670(
        address target,
        uint256 value,
        bytes32 context,
        bytes callData,
        Placeholder[] placeholders
    );
}

/// IExecutor transfer function (separate sol! to avoid naming conflicts).
pub(crate) mod executor_sol {
    use alloy_sol_types::sol;

    sol! {
        function transfer(address recipient, uint256 amount);
    }
}
