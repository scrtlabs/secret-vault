use cosmwasm_std::{Storage, StdResult, StdError};
use serde::{Deserialize, Serialize};

pub const SEED_KEY: &[u8] = "seed".as_bytes();


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


pub fn store_key_record<S: Storage>(storage: &mut S, key_id: &String, private_key: [u8; 32], api_key: &String, passphrase: &String) {

    let record = PrivateKeyRecord{
        api_key: api_key.clone(),
        passphrase: passphrase.clone(),
        key: private_key
    };

    let record_bytes: Vec<u8> = bincode2::serialize(&record).unwrap();

    storage.set(&key_id.as_bytes(), record_bytes.as_slice());
}

pub fn get_key_record<S: Storage>(storage: &mut S, key_id: &String) -> StdResult<PrivateKeyRecord> {
    if let Some(record_bytes) = storage.get(&key_id.as_bytes()) {
        let record: PrivateKeyRecord = bincode2::deserialize(&record_bytes).unwrap();
        Ok(record)
    } else {
        Err(StdError::GenericErr { msg: "Key ID not found".to_string(), backtrace: None })
    }
}