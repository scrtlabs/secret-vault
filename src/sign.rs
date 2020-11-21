use secp256k1;

pub fn pubkey(priv_key: &[u8; 32]) -> secp256k1::PublicKey {
    let pk = secp256k1::SecretKey::parse(priv_key).unwrap();
    secp256k1::PublicKey::from_secret_key(&pk)
}
