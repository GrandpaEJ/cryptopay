//! Payment utility functions for amount conversion and comparison

use crate::error::{Error, Result};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

/// Convert wei to BNB/ether
pub fn wei_to_ether(wei: u128) -> Decimal {
    Decimal::from(wei) / Decimal::from(1_000_000_000_000_000_000u128)
}

/// Convert BNB/ether to wei
pub fn ether_to_wei(ether: Decimal) -> u128 {
    (ether * Decimal::from(1_000_000_000_000_000_000u128))
        .to_u128()
        .unwrap_or(0)
}

/// Convert gwei to wei
pub fn gwei_to_wei(gwei: Decimal) -> u128 {
    (gwei * Decimal::from(1_000_000_000u128))
        .to_u128()
        .unwrap_or(0)
}

/// Convert wei to gwei
pub fn wei_to_gwei(wei: u128) -> Decimal {
    Decimal::from(wei) / Decimal::from(1_000_000_000u128)
}

/// Parse token amount from string with custom decimals
///
/// # Example
/// ```
/// # use cryptopay::payment::utils::parse_token_amount;
/// let amount = parse_token_amount("1000000000000000000", 18).unwrap();
/// assert_eq!(amount, 1000000000000000000u128);
/// ```
pub fn parse_token_amount(amount: &str, _decimals: u8) -> Result<u128> {
    amount
        .parse()
        .map_err(|_| Error::generic(format!("Invalid token amount: {}", amount)))
}

/// Format token amount to string with custom decimals
///
/// Converts raw token amount (in smallest unit) to human-readable format
pub fn format_token_amount(amount: u128, decimals: u8) -> String {
    let divisor = 10u128.pow(decimals as u32);
    let whole = amount / divisor;
    let fractional = amount % divisor;

    if fractional == 0 {
        whole.to_string()
    } else {
        format!("{}.{:0width$}", whole, fractional, width = decimals as usize)
    }
}

/// Convert token amount (human-readable) to raw units
pub fn token_to_raw(amount: Decimal, decimals: u8) -> u128 {
    let multiplier = 10u128.pow(decimals as u32);
    (amount * Decimal::from(multiplier)).to_u128().unwrap_or(0)
}

/// Convert raw token units to human-readable amount
pub fn raw_to_token(raw_amount: u128, decimals: u8) -> Decimal {
    let divisor = 10u128.pow(decimals as u32);
    Decimal::from(raw_amount) / Decimal::from(divisor)
}

/// Compare two amounts with tolerance
///
/// Returns true if the actual amount is within tolerance of expected amount.
/// Tolerance is a percentage (e.g., 0.01 = 1%)
pub fn amounts_match(expected: Decimal, actual: Decimal, tolerance_percent: Decimal) -> bool {
    if expected == Decimal::ZERO {
        return actual == Decimal::ZERO;
    }

    let diff = if actual > expected {
        actual - expected
    } else {
        expected - actual
    };

    let tolerance_amount = expected * tolerance_percent / Decimal::from(100);
    diff <= tolerance_amount
}

/// Check if actual amount meets or exceeds expected (allowing small under-payment)
pub fn amount_sufficient(expected: Decimal, actual: Decimal, min_percent: Decimal) -> bool {
    let min_required = expected * min_percent / Decimal::from(100);
    actual >= min_required
}

/// Validate Ethereum/BSC address format
pub fn is_valid_address(address: &str) -> bool {
    if !address.starts_with("0x") {
        return false;
    }

    if address.len() != 42 {
        return false;
    }

    address[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate transaction hash format
pub fn is_valid_tx_hash(hash: &str) -> bool {
    if !hash.starts_with("0x") {
        return false;
    }

    if hash.len() != 66 {
        return false;
    }

    hash[2..].chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_wei_ether_conversion() {
        let wei = 1_000_000_000_000_000_000u128;
        let ether = wei_to_ether(wei);
        assert_eq!(ether, Decimal::from(1));

        let wei_back = ether_to_wei(ether);
        assert_eq!(wei_back, wei);
    }

    #[test]
    fn test_token_conversions() {
        let raw = token_to_raw(Decimal::from(100), 18);
        let back = raw_to_token(raw, 18);
        assert_eq!(back, Decimal::from(100));
    }

    #[test]
    fn test_amounts_match() {
        let expected = Decimal::from(100);
        let actual = Decimal::from_str("100.5").unwrap();
        let tolerance = Decimal::from(1); // 1%

        assert!(amounts_match(expected, actual, tolerance));

        let actual_far = Decimal::from(110);
        assert!(!amounts_match(expected, actual_far, tolerance));
    }

    #[test]
    fn test_amount_sufficient() {
        let expected = Decimal::from(100);
        let actual = Decimal::from(99); // 99% of expected
        let min_percent = Decimal::from(95); // Allow 95% minimum

        assert!(amount_sufficient(expected, actual, min_percent));

        let actual_low = Decimal::from(90); // Only 90%
        assert!(!amount_sufficient(expected, actual_low, min_percent));
    }

    #[test]
    fn test_address_validation() {
        assert!(is_valid_address(
            "0x1234567890123456789012345678901234567890"
        ));
        assert!(!is_valid_address("1234567890123456789012345678901234567890")); // No 0x
        assert!(!is_valid_address("0x123")); // Too short
        assert!(!is_valid_address(
            "0xGGGG567890123456789012345678901234567890"
        )); // Invalid hex
    }

    #[test]
    fn test_tx_hash_validation() {
        assert!(is_valid_tx_hash(
            "0x1234567890123456789012345678901234567890123456789012345678901234"
        ));
        assert!(!is_valid_tx_hash(
            "1234567890123456789012345678901234567890123456789012345678901234"
        )); // No 0x
        assert!(!is_valid_tx_hash("0x123")); // Too short
    }
}
