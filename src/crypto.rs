use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, Key, KeyInit, OsRng},
    Aes256GcmSiv, // Or `Aes128GcmSiv`
    Nonce,
};
use rand::RngCore;

/// Generate a random nonce with 96bit fixed size.
pub fn generate_nonce() -> Nonce {
    let mut rand = [0u8; 12];
    OsRng.fill_bytes(&mut rand);
    let nonce = Nonce::from_iter(rand);
    nonce
}

// Generate a 256bit key
pub fn generate_key() -> Key<Aes256GcmSiv> {
    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    key
}

/// Encrypt a plain text msg
pub fn encrypt(msg: &str, key: &Key<Aes256GcmSiv>, nonce: &Nonce) -> Vec<u8> {
    let cipher = Aes256GcmSiv::new(key);
    let ciphertext = cipher.encrypt(nonce, msg.as_bytes()).unwrap();
    ciphertext
}

/// Decrypt the byte stream
pub fn decrypt(ciphertext: &Vec<u8>, key: &Vec<u8>, nonce: &Vec<u8>) -> String {
    let cipher = Aes256GcmSiv::new_from_slice(&key).expect("Key was incorrect");
    let nonce_ga: GenericArray<u8, _> = GenericArray::clone_from_slice(nonce);
    // let cipher = Aes256GcmSiv::new(key);
    let msg = cipher.decrypt(&nonce_ga, ciphertext.as_ref()).unwrap();
    String::from_utf8(msg).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_generate_nonce() {
        let nonce = generate_nonce();
        assert!(true)
    }

    #[test]
    fn test_generate_key() {
        let key = generate_key();
        assert!(true)
    }

    #[test]
    fn test_encrypt() {
        let key = generate_key();
        let nonce = generate_nonce();
        let enc = encrypt(
            "A message with-a-hyphen and some ⟨ä⟩, ⟨ö⟩, and ⟨ü⟩ and 😀!??",
            &key,
            &nonce,
        );
        assert!(true)
    }

    #[test]
    fn test_decrypt() {
        let key = generate_key();
        let nonce = generate_nonce();
        let msg_in = "A message with-a-hyphen and some ⟨ä⟩, ⟨ö⟩, and ⟨ü⟩ and 😀!??";
        let enc = encrypt(&msg_in, &key, &nonce);

        // Key and Nonce will return as Vec<u8>
        let key_vec: Vec<u8> = key.into_iter().collect();
        let nonce_vec: Vec<u8> = nonce.into_iter().collect();

        let msg_out = decrypt(&enc, &key_vec, &nonce_vec);
        assert_eq!(msg_out, msg_in.to_string())
    }
}
