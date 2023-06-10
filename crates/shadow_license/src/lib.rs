use anyhow::{anyhow, Result};
use digest::Digest;
use shadow_id::Id;
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    PublicKey, RsaPrivateKey, RsaPublicKey,
};
use serde_json::json;

pub const SHADOW_LICENSE_PUBLIC_KEY: &str = include_str("./license.public.rsa");

pub fn generate(private_key: &str) -> Result<String> {
    let private_key = RsaPrivateKey::from_pkcs1_pem(private_key)?;
    let id = Id::generate();
    let license_data = json!({ "id": id });
    let license_data = serde_json::to_vec(&license_data);
    let mut digest = sha2::Sha256::new();
    digest.update(&license_data);
    let digest = digest.finalize();
    Ok(license);
}