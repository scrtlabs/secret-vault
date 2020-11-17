use core::mem;
use cosmwasm_std::Env;
use secret_toolkit::crypto::{sha_256, Prng, SHA256_HASH_SIZE};

use crate::state::PrivateKeyRecord;

pub fn generate_api_key(seed: &[u8], env: &Env) -> String {
    let height_slice = unsafe { mem::transmute::<u64, [u8; 8]>(env.block.height) };

    let mut entropy: Vec<u8> = env.message.sender.0.as_bytes().to_vec();
    entropy.extend_from_slice(height_slice.as_ref());
    
   let mut rng = Prng::new(seed, &entropy);
    rng.set_word_pos((SHA256_HASH_SIZE / 2) as u32);
    "api_key_".to_string() + &base64::encode(rng.rand_bytes())
}

pub fn authenticate_request(
    record: &PrivateKeyRecord,
    api_key: &String,
    passphrase: &String,
) -> bool {
    return &record.api_key == api_key && &record.passphrase == passphrase;
}

pub fn validate_data_len(data: &[u8]) -> bool {
    data.len() == SHA256_HASH_SIZE
}

pub fn generate_seed(keying_material: &String) -> [u8; 32] {
    sha_256(&keying_material.as_bytes())
}

pub fn generate_key_id(env: &Env) -> String {
    let entropy = unsafe { mem::transmute::<u64, [u8; 8]>(env.block.height) };

    let mut rng = Prng::new(env.message.sender.0.as_bytes(), entropy.as_ref());
    rng.set_word_pos(0);
    "key_".to_string() + &base64::encode(rng.rand_bytes())
}

pub fn generate_private_key(env: &Env, seed: &[u8], entropy: &[u8]) -> [u8; 32] {
    let height_slice = unsafe { mem::transmute::<u64, [u8; 8]>(env.block.height) };

    let mut keying_material: Vec<u8> = env.message.sender.0.as_bytes().to_vec();
    keying_material.extend_from_slice(height_slice.as_ref());
    keying_material.extend_from_slice(entropy);
   
    let mut rng = Prng::new(seed, &keying_material);
    rng.set_word_pos(0);
    rng.rand_bytes()
}
