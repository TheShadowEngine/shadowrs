#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
        let private_key = rsa::RsaPrivateKey::new(&mut rand::thread_rng(), 2048).unwrap();
        let public_key = rsa::RsaPublicKey::from(&private_key);
        let private_key = private_key
            .to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
            .unwrap();
        let public_key = public_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF).unwrap();
        let license = generate(&private_key).unwrap();
        assert!(verify(&license, &public_key).unwrap());
    }
}