use alloy_primitives::U256;

const PERCENTAGE_FACTOR: U256 = U256::from_limbs([10_000, 0, 0, 0]);
const HALF_PERCENTAGE_FACTOR: U256 = U256::from_limbs([5_000, 0, 0, 0]);

/// Equivalent to Aave's PercentageMath.percentMul.
///
/// Returns `(value * percentage + HALF_PERCENTAGE_FACTOR) / PERCENTAGE_FACTOR`.
pub fn percent_mul(value: U256, percentage: U256) -> U256 {
    (value * percentage + HALF_PERCENTAGE_FACTOR) / PERCENTAGE_FACTOR
}

/// Equivalent to `mulDivUp(x, y, d) = (x * y + d - 1) / d`.
///
/// Used for UniswapV3 fee calculations.
pub fn mul_div_up(x: U256, y: U256, d: U256) -> U256 {
    (x * y + d - U256::from(1)) / d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_mul() {
        // 1_000_000 * 5 / 10_000 = 500
        let result = percent_mul(U256::from(1_000_000u64), U256::from(5u64));
        assert_eq!(result, U256::from(500u64));
    }

    #[test]
    fn test_percent_mul_rounding() {
        // 1 * 1 + 5000 / 10000 = 5001 / 10000 = 0 (rounds down)
        let result = percent_mul(U256::from(1u64), U256::from(1u64));
        assert_eq!(result, U256::ZERO);

        // 10001 * 1 + 5000 / 10000 = 15001 / 10000 = 1
        let result = percent_mul(U256::from(10001u64), U256::from(1u64));
        assert_eq!(result, U256::from(1u64));
    }

    #[test]
    fn test_mul_div_up() {
        // 1_000_000 * 500 / 1_000_000 = 500
        let result = mul_div_up(
            U256::from(1_000_000u64),
            U256::from(500u64),
            U256::from(1_000_000u64),
        );
        assert_eq!(result, U256::from(500u64));
    }

    #[test]
    fn test_mul_div_up_rounds_up() {
        // 1 * 1 / 3 => (1 + 2) / 3 = 1 (rounds up from 0.333...)
        let result = mul_div_up(U256::from(1u64), U256::from(1u64), U256::from(3u64));
        assert_eq!(result, U256::from(1u64));
    }
}
