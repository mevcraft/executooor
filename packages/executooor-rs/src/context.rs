use alloy_primitives::{Address, B256};

/// Encodes a callback context as a `bytes32` value.
///
/// The upper 12 bytes hold the `data_index` and the lower 20 bytes hold the `sender` address.
///
/// This matches the TypeScript encoding:
/// ```js
/// "0x" + dataIndex.toString(16).padStart(24, "0") + sender.substring(2)
/// ```
///
/// And the Solidity decoding:
/// ```solidity
/// uint256 dataIndex = uint256(context >> 160);
/// address sender = address(uint160(uint256(context)));
/// ```
pub fn encode_context(sender: Address, data_index: u64) -> B256 {
    // Layout (big-endian, 32 bytes total):
    //   [0..4]   padding (zeros, since data_index fits in u64)
    //   [4..12]  data_index as u64 big-endian  ← Solidity: context >> 160
    //   [12..32] sender address (20 bytes)     ← Solidity: address(uint160(context))
    let mut b = [0u8; 32];
    b[4..12].copy_from_slice(&data_index.to_be_bytes());
    b[12..32].copy_from_slice(sender.as_slice());
    B256::from(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, U256};

    #[test]
    fn test_encode_context_zero() {
        let ctx = encode_context(Address::ZERO, 0);
        assert_eq!(ctx, B256::ZERO);
    }

    #[test]
    fn test_encode_context_with_index() {
        let sender = address!("BA12222222228d8Ba445958a75a0704d566BF2C8");
        let ctx = encode_context(sender, 3);
        // Upper 12 bytes: 000000000000000000000003
        // Lower 20 bytes: BA12222222228d8Ba445958a75a0704d566BF2C8
        let hex = format!("{ctx:?}");
        assert!(hex.contains("000000000000000000000003"));
        assert!(hex
            .to_lowercase()
            .contains("ba12222222228d8ba445958a75a0704d566bf2c8"));
    }

    #[test]
    fn test_encode_context_roundtrip() {
        let sender = address!("1234567890abcdef1234567890abcdef12345678");
        let data_index = 42u64;
        let ctx = encode_context(sender, data_index);

        // Decode like the contract does
        let ctx_u256 = U256::from_be_bytes(ctx.0);
        let decoded_index = ctx_u256 >> 160;
        let decoded_sender_u256 = ctx_u256
            & U256::from_be_bytes({
                let mut mask = [0u8; 32];
                mask[12..32].fill(0xff);
                mask
            });
        let mut sender_bytes = [0u8; 20];
        sender_bytes.copy_from_slice(&decoded_sender_u256.to_be_bytes::<32>()[12..32]);

        assert_eq!(decoded_index, U256::from(data_index));
        assert_eq!(Address::from(sender_bytes), sender);
    }
}
