//! This integration test tries to run and call the generated wasm.
//! It depends on a Wasm build being available, which you can create with `cargo wasm`.
//! Then running `cargo integration-test` will validate we can properly call into that generated Wasm.
//!
//! You can easily convert unit tests to integration tests.
//! 1. First copy them over verbatum,
//! 2. Then change
//!      let mut deps = mock_dependencies(20, &[]);
//!    to
//!      let mut deps = mock_instance(WASM, &[]);
//! 3. If you access raw storage, where ever you see something like:
//!      deps.storage.get(CONFIG_KEY).expect("no data stored");
//!    replace it with:
//!      deps.with_storage(|store| {
//!          let data = store.get(CONFIG_KEY).expect("no data stored");
//!          //...
//!      });
//! 4. Anywhere you see query(&deps, ...) you must replace it with query(&mut deps, ...)

use cosmwasm_std::testing::{mock_dependencies, mock_env};
use cosmwasm_std::{coins, HandleResponse};

use secret_vault::contract::{handle, init};
use secret_vault::msg::{HandleMsg, InitMsg};

// This line will test the output of cargo wasm
//static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/secret_vault.wasm");
// You can uncomment this line instead to test productionified build from rust-optimizer
// static WASM: &[u8] = include_bytes!("../contract.wasm");

#[test]
fn init_sign() {
    let mut deps = mock_dependencies(20, &[]);
    let seed_phrase = "GX4CRn8cLOtaxWAduRKr";
    let key_seed = "uI67K87zFSU3WKlXtiYK";
    let passphrase = "mmSnja3rrA8J9NWImqOz";
    let data = "674de7af309b691bc121e6f40a6c76a513883925f5358838fc5943db720a6e78";

    let msg = InitMsg::Init {
        seed_phrase: seed_phrase.to_string(),
    };
    let env = mock_env("creator", &coins(1000, "earth"));
    let res = init(&mut deps, env, msg);
    println!("{:?}", res.unwrap());

    let msg = HandleMsg::NewKey {
        key_seed: key_seed.to_string(),
        passphrase: passphrase.to_string(),
    };
    let env = mock_env("creator", &coins(1000, "earth"));
    let res = handle(&mut deps, env, msg);
    let new_key_res = res.unwrap();
    let api_key = get_log_attribute(&new_key_res, "api_key").expect("No api key detected");
    let key_id = get_log_attribute(&new_key_res, "key_id").expect("No key id detected");

    let msg = HandleMsg::Sign {
        passphrase: passphrase.to_string(),
        data: data.to_string(),
        api_key,
        key_id,
    };
    let env = mock_env("creator", &coins(1000, "earth"));
    let res = handle(&mut deps, env, msg);
    let sign_res = res.unwrap();
    let signature = get_log_attribute(&sign_res, "signature").expect("No signature detected");

    let expected_signature = "e3eb2ff3403a0dbb55253dc2039995eaf4932d92b55c8422f69b5b5e1f0753c15581f9c53e3987db54d6f3f04d8c9f32407652758456411a984f55d8c1c6097a";
    assert_eq!(signature, expected_signature);
}

fn get_log_attribute(resp: &HandleResponse, key: &str) -> Option<String> {
    for log in resp.log.iter() {
        if &log.key == key {
            return Some(log.value.to_owned());
        }
    }

    None
}
