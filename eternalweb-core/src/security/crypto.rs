//! Additional cryptographic utilities

use super::*;

/// Sign data with a private key
pub fn sign_data(data: &[u8], priv_key: &PrivateKey) -> Result<Vec<u8>> {
    match priv_key.key_type {
        KeyType::Ed25519 => sign_ed25519(data, priv_key),
        KeyType::Dilithium => sign_dilithium(data, priv_key),
        _ => Err(Error::Crypto("Unsupported key type for signing".to_string())),
    }
}

/// Verify signature
pub fn verify_signature(data: &[u8], signature: &[u8], pub_key: &PublicKey) -> Result<bool> {
    match pub_key.key_type {
        KeyType::Ed25519 => verify_ed25519(data, signature, pub_key),
        KeyType::Dilithium => verify_dilithium(data, signature, pub_key),
        _ => Err(Error::Crypto("Unsupported key type for verification".to_string())),
    }
}

fn sign_ed25519(data: &[u8], priv_key: &PrivateKey) -> Result<Vec<u8>> {
    use ed25519_dalek::{Signature, Signer, SigningKey};

    let signing_key = SigningKey::from_bytes(
        priv_key.bytes.as_slice().try_into()
            .map_err(|_| Error::Crypto("Invalid private key".to_string()))?
    );

    let signature: Signature = signing_key.sign(data);
    Ok(signature.to_bytes().to_vec())
}

fn verify_ed25519(data: &[u8], signature: &[u8], pub_key: &PublicKey) -> Result<bool> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let verifying_key = VerifyingKey::from_bytes(
        pub_key.bytes.as_slice().try_into()
            .map_err(|_| Error::Crypto("Invalid public key".to_string()))?
    )
    .map_err(|e| Error::Crypto(format!("Invalid verifying key: {:?}", e)))?;

    let sig = Signature::from_bytes(
        signature.try_into()
            .map_err(|_| Error::Crypto("Invalid signature".to_string()))?
    );

    Ok(verifying_key.verify(data, &sig).is_ok())
}

fn sign_dilithium(_data: &[u8], _priv_key: &PrivateKey) -> Result<Vec<u8>> {
    // TODO: Implement Dilithium signing
    Ok(vec![])
}

fn verify_dilithium(_data: &[u8], _signature: &[u8], _pub_key: &PublicKey) -> Result<bool> {
    // TODO: Implement Dilithium verification
    Ok(true)
}
