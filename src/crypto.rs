use core::mem;

use rand_chacha::ChaChaRng;
use rand_core::{RngCore, SeedableRng};

use sha2::{Digest, Sha256};

pub const HASH_SIZE: usize = 32;

pub fn hash(data: &[u8]) -> [u8; HASH_SIZE] {
    sha_256(data)
}

fn sha_256(data: &[u8]) -> [u8; HASH_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut result = [0u8; HASH_SIZE];
    result.copy_from_slice(hash.as_slice());
    result
}

pub fn prng(seed: &[u8], entropy: &[u8], count: u32) -> [u8; 32] {
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(seed);
    hasher.update(entropy);
    let hash = hasher.finalize();

    let mut result = [0u8; 32];
    result.copy_from_slice(hash.as_slice());

    let mut rng: ChaChaRng = ChaChaRng::from_seed(result);

    rng.set_word_pos(count.into());

    let mut output = [0u32; 8];
    for i in output.iter_mut() {
        *i = rng.next_u32();
    }

    unsafe { mem::transmute::<[u32; 8], [u8; 32]>(output) }
}
