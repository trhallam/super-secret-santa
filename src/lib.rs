mod error;
mod participant;
mod secretsanta;
mod utils;

use error::SecretSantaError;
use secretsanta::SecretSanta;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

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
    match serde_wasm_bindgen::to_value(&pairings) {
        Ok(v) => return Ok(v),
        Err(_) => return Err(SecretSantaError::new("Serialisation error".to_string())),
    }
}

#[cfg(test)]
mod tests {

    use crate::get_secret_santas;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_get_secret_santa() {
        let instructions = "Amy\nTom !Amy\nBen =Amy\n";

        let pairings = get_secret_santas(instructions.to_string()).unwrap();
    }
}
