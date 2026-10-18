use crate::error::MppError;

/// The network passphrase for mainnet — shared by x402 and MPP.
pub const PUBNET_PASSPHRASE: &str = "Public Global Stellar Network ; September 2015";
/// The network passphrase for testnet — shared by x402 and MPP.
pub const TESTNET_PASSPHRASE: &str = "Test SDF Network ; September 2015";

/// The CAIP-2 identifier for a network passphrase.
///
/// # Errors
///
/// Returns [`MppError::UnsupportedNetwork`] for any passphrase other than
/// mainnet's or testnet's.
pub fn caip2_for_passphrase(network_passphrase: &str) -> Result<&'static str, MppError> {
    match network_passphrase {
        PUBNET_PASSPHRASE => Ok("stellar:pubnet"),
        TESTNET_PASSPHRASE => Ok("stellar:testnet"),
        other => Err(MppError::UnsupportedNetwork {
            network: other.to_string(),
        }),
    }
}
