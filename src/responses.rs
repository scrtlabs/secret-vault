use cosmwasm_std::{log, HandleResponse};

use hex;

#[derive(Clone)]
pub struct CreateKeyResponse {
    pub key_id: String,
    pub api_key: String,
    pub public_key: [u8; 33],
}

#[derive(Clone)]
pub struct SignResponse {
    pub signature: [u8; 64],
}

impl Into<HandleResponse> for SignResponse {
    fn into(self) -> HandleResponse {
        let sig = hex::encode(self.signature.as_ref());

        HandleResponse {
            messages: vec![],
            log: vec![log("signature", sig)],
            data: None,
        }
    }
}

impl Into<HandleResponse> for CreateKeyResponse {
    fn into(self) -> HandleResponse {
        let pubkey = hex::encode(self.public_key.as_ref());

        HandleResponse {
            messages: vec![],
            log: vec![
                log("api_key", self.api_key),
                log("key_id", self.key_id),
                log("public_key", pubkey),
            ],
            data: None,
        }
    }
}

impl Returnable for CreateKeyResponse {}
impl Returnable for SignResponse {}

trait Returnable
where
    Self: Into<HandleResponse>,
{
}
