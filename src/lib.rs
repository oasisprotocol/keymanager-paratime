//! The Cipher key manager.
use std::collections::HashSet;

use oasis_core_keymanager::policy::TrustedSigners;
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
pub fn trusted_policy_signers() -> TrustedSigners {
    if is_testnet() {
        // Testnet.
        TrustedSigners {
            signers: HashSet::from([
                "b27b3d0245d4cbd78be8e04e473f36abee350fcfbc438000313db1bb06117a43".into(),
                "c37cbd0345965fda84fbaa372a01fc840b7b66eebfeb66dfdd35bb3e801f2cf3".into(),
                "df8ca9fc78ce2c01f8217e8ce7aa582e8952545f412426fe07d42ca119e12166".into(),
            ]),
            threshold: 2,
        }
    } else {
        // Mainnet.
        TrustedSigners {
            signers: HashSet::from([
                "26768b7918cfef10cf2659c3da6bf7f8f76e216257ab08e966c698b95d01e40d".into(),
                "2b9d615eb3e8f2ca46e908575bfe6bab0d9f9aadd47af06c40a9d195c4c41a45".into(),
                "4b7ff0a68daf4ef3d8e6d6a277358fab582f55761c1ae028d9d7cb20883b3520".into(),
                "982226ad74da8ef2502a8204f07dc8e2ad4245646b6fb146a4c8eaf3410c2b29".into(),
                "ef6dd40d7dea169885c002957b2fc6d29bc474ff3cdb59360f16f90bf6dd1c71".into(),
            ]),
            threshold: 3,
        }
    }
}

/// Trust root.
pub fn trust_root() -> TrustRoot {
    if is_testnet() {
        // Testnet.
        TrustRoot {
            height: 21135326,
            hash: "af3b54e6aed1549df7a4ae2a0903cb5f4e94589fc59dc57ea86aa5bdf47150fd".into(),
            runtime_id: "4000000000000000000000000000000000000000000000004a1a53dff2ae482d".into(),
            chain_context: "0b91b8e4e44b2003a7c5e23ddadb5e14ef5345c0ebcb3ddcae07fa2f244cab76"
                .to_string(),
        }
    } else {
        // Mainnet.
        TrustRoot {
            height: 19431129,
            hash: "b11681139347a774bd863e7f29bd6743422b0bff495f9f3175132bea4a6d668b".into(),
            runtime_id: "4000000000000000000000000000000000000000000000008c5ea5e49b4bc9ac".into(),
            chain_context: "bb3d748def55bdfb797a2ac53ee6ee141e54cd2ab2dc2375f4a0703a178e6e55"
                .to_string(),
        }
    }
}
