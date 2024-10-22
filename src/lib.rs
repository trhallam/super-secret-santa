mod crypto;
mod error;
mod participant;
mod secretsanta;
mod utils;

use base64ct::{Base64Url, Encoding};
use error::SecretSantaError;
use secretsanta::SecretSanta;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct EncryptedSecretSanta {
    key: String,
    nonce: String,
    pairing: String,
}

/// Encrypt secret santas returning the key, nonce and ciphertext in b64.
fn encrypt_secret_santa(paired_with: &str) -> EncryptedSecretSanta {
    let key = crypto::generate_key();
    let nonce = crypto::generate_nonce();
    let ciphertext = crypto::encrypt(paired_with, &key, &nonce);
    let enc_ss = EncryptedSecretSanta {
        key: Base64Url::encode_string(&key),
        nonce: Base64Url::encode_string(&nonce),
        pairing: Base64Url::encode_string(&ciphertext),
    };
    enc_ss
}

fn decode_vec(input: &str) -> Result<Vec<u8>, SecretSantaError> {
    match Base64Url::decode_vec(input) {
        Ok(v) => Ok(v),
        Err(_) => Err(SecretSantaError::new(
            "Could not decode secrets".to_string(),
        )),
    }
}
// converts
#[wasm_bindgen(catch)]
pub fn decrypt_secret_santa(
    key: &str,
    nonce: &str,
    ciphertext: &str,
) -> Result<String, SecretSantaError> {
    let dc_key = decode_vec(key)?;
    let dc_nonce = decode_vec(nonce)?;
    let dc_ct = decode_vec(ciphertext)?;
    let name = crypto::decrypt(&dc_ct, &dc_key, &dc_nonce);
    Ok(name)
}

/// Create secret santa pairs
/// Takes a set of instructions as a line break delimited string.
#[wasm_bindgen(catch)]
pub fn get_secret_santas(instructions: String) -> Result<JsValue, SecretSantaError> {
    let mut secret_santa = SecretSanta::new();

    // loop all lines
    for i in instructions.trim().split('\n') {
        secret_santa.add_instruction(&i)?;
    }
    secret_santa.generate_pairings()?;

    let pairings = secret_santa.get_pairings();
    let enc_pairings: HashMap<String, EncryptedSecretSanta> = pairings
        .iter()
        .map(|(k, v)| (k.clone(), encrypt_secret_santa(v)))
        .collect();

    match serde_wasm_bindgen::to_value(&enc_pairings) {
        Ok(v) => return Ok(v),
        Err(_) => return Err(SecretSantaError::new("Serialisation error".to_string())),
    }
}

#[cfg(test)]
mod tests {

    use crate::{decrypt_secret_santa, encrypt_secret_santa, get_secret_santas};
    use wasm_bindgen_test::*;

    #[test]
    fn test_encrypt_secret_santa() {
        let enc = encrypt_secret_santa("Tom");
        println!("{}", enc.nonce);
    }

    #[wasm_bindgen_test]
    fn test_decrypt_secret_santa() {
        let enc = encrypt_secret_santa("Tom");
        let name = decrypt_secret_santa(&enc.key, &enc.nonce, &enc.pairing).unwrap();
        assert_eq!(name, "Tom".to_string())
    }

    #[wasm_bindgen_test]
    fn test_get_secret_santa() {
        let instructions = "Amy\nTom !Amy\nBen =Amy\n";

        let pairings = get_secret_santas(instructions.to_string()).unwrap();
    }
}
