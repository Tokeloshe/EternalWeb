//! Ethical sharing mechanisms

use super::*;

/// License types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum License {
    PublicDomain,
    CreativeCommons(String),
    Custom(String),
}

/// Check license compliance
pub fn check_license_compliance(license: &License, intended_use: &str) -> bool {
    match license {
        License::PublicDomain => true,
        License::CreativeCommons(variant) => {
            // TODO: Parse CC variant and check compliance
            true
        }
        License::Custom(terms) => {
            // TODO: Parse custom terms
            true
        }
    }
}

/// Share content with license verification
pub async fn share_with_license_check(
    cid: &Cid,
    recipient: &crate::security::PublicKey,
    license: &License,
) -> Result<()> {
    // Check if sharing is allowed
    if !check_license_compliance(license, "sharing") {
        return Err(Error::Other("License does not permit sharing".to_string()));
    }

    ethical_share(cid, recipient).await
}
