use cosmwasm_std::{StdError, StdResult, Storage};
use serde::{Deserialize, Serialize};

pub const SEED_KEY: &[u8] = b"seed";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PrivateKeyRecord {
    pub key: [u8; 32],
    pub api_key: String,
    pub passphrase: String,
}

pub fn store_seed<S: Storage>(storage: &mut S, seed: [u8; 32]) {
    storage.set(&SEED_KEY, &seed);
}

pub fn get_seed<S: Storage>(storage: &mut S) -> Vec<u8> {
    storage.get(&SEED_KEY).unwrap()
}

pub fn store_key_record<S: Storage>(
    storage: &mut S,
    key_id: &str,
    private_key: [u8; 32],
    api_key: String,
    passphrase: String,
) {
    let record = PrivateKeyRecord {
        api_key,
        passphrase,
        key: private_key,
    };

    let record_bytes: Vec<u8> = bincode2::serialize(&record).unwrap();

    storage.set(key_id.as_bytes(), record_bytes.as_slice());
}

pub fn get_key_record<S: Storage>(storage: &mut S, key_id: &str) -> StdResult<PrivateKeyRecord> {
    if let Some(record_bytes) = storage.get(&key_id.as_bytes()) {
        let record: PrivateKeyRecord = bincode2::deserialize(&record_bytes).unwrap();
        Ok(record)
    } else {
        Err(StdError::generic_err("Key ID not found"))
    }
}
