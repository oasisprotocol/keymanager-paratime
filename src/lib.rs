//! The Cipher key manager.
use std::collections::HashSet;

use oasis_core_keymanager::policy::TrustedPolicySigners;
use oasis_core_runtime::consensus::verifier::TrustRoot;

/// Determine whether the build is for Testnet.
///
/// If the crate version has a pre-release component (e.g. 2.4.0-testnet) then the build is
/// classified as Testnet. If there is no such component (e.g. 2.4.0) then it is classified as
/// Mainnet.
const fn is_testnet() -> bool {
    !env!("CARGO_PKG_VERSION_PRE").is_empty()
}

/// Trusted key manager policy signer set.
pub fn trusted_policy_signers() -> TrustedPolicySigners {
    if is_testnet() {
        // Testnet.
        TrustedPolicySigners {
            signers: HashSet::from([
                "b27b3d0245d4cbd78be8e04e473f36abee350fcfbc438000313db1bb06117a43".into(),
                "c37cbd0345965fda84fbaa372a01fc840b7b66eebfeb66dfdd35bb3e801f2cf3".into(),
                "df8ca9fc78ce2c01f8217e8ce7aa582e8952545f412426fe07d42ca119e12166".into(),
            ]),
            threshold: 2,
        }
    } else {
        // Mainnet.
        panic!("key manager policy signer set not defined for Mainnet");
    }
}

/// Trust root.
pub fn trust_root() -> TrustRoot {
    if is_testnet() {
        // Testnet.
        TrustRoot {
            height: 12497936,
            hash: "ce19521b7eb4c6dddc9592b9b26d90f3a5ad14bd1b807478b06739bd069ef262".into(),
            runtime_id: "4000000000000000000000000000000000000000000000004a1a53dff2ae482d".into(),
            chain_context: "50304f98ddb656620ea817cc1446c401752a05a249b36c9b90dba4616829977a"
                .to_string(),
        }
    } else {
        panic!("key manager trust root not defined for Mainnet");
    }
}
