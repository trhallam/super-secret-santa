mod error;
mod participant;
mod secretsanta;
mod utils;

use error::SecretSantaError;
use orion::aead;
use secretsanta::SecretSanta;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

const KEY: &[u8] = b"don't give up your secret santa";

#[wasm_bindgen(catch)]
pub fn get_secret_santas(instructions: String) -> Result<JsValue, SecretSantaError> {
    let mut secrets: HashMap<String, String> = HashMap::new();

    let mut secret_santa = SecretSanta::new();

    for i in instructions.trim().split('\n') {
        secret_santa.add_instruction(&i)?;
    }
    secret_santa.generate_pairings()?;

    let pairings = secret_santa.get_pairings();

    // encrypt the pairings
    let key = aead::SecretKey::from_slice(&KEY).unwrap();

    // TODO FINISH / FIX THIS
    // let secrets: HashMap<String, Vec<u8>> = pairings
    //     .iter()
    //     .map(|(k, Some(val))| {
    //         (
    //             k.clone(),
    //             aead::seal(&key, &val.clone().as_bytes()).unwrap(),
    //         )
    //     })
    //     .collect();

    // for (_, val) in pairings.iter_mut() {

    // }
    //     |
    // )
    // let cipher = Aes256CbcEnc:: ::new_from_slices(KEY, &IV).unwrap();

    Ok(serde_wasm_bindgen::to_value(&secrets).unwrap())
}
// pub use secretsanta::SecretSanta;
// use regex::Regex;
// use std::collections::HashMap;
// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub struct SecretSanta {
//     // These variables cannot be public because wasm_bindgen doesn't understand how to serialise complex rust types for
//     // wasm.
//     names: Vec<String>,
//     enforce: HashMap<String, String>,
//     block: HashMap<String, Vec<String>>,
// }

// #[wasm_bindgen]
// impl SecretSanta {
//     /// Initialises a SecretSanta
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// let mut santa = SecretSanta::new();
//     /// ```
//     pub fn new() -> SecretSanta {
//         let names = Vec::new();
//         let enforce = HashMap::new();
//         let block = HashMap::new();

//         return SecretSanta {
//             names,
//             enforce,
//             block,
//         };
//     }

//     /// Adds a name to the SecretSanta recipient list
//     // #[wasm_bindgen(catch)]
//     pub fn add_name(&mut self, name: String) -> Result<(), JsValue> {
//         if self.names.iter().any(|n| n == &name) {
//             let msg = format!("recipient name already exists {name}");
//             return Err(JsValue::from("duplicate recipient in input"));
//         } else {
//             self.names.push(name);
//         }
//         return Ok(());
//     }

//     /// Adds an enforced match criterion between a recipient and giver
//     pub fn enforce_match(&mut self, recipient: String, giver: String) {
//         self.enforce.insert(recipient, giver);
//     }

//     /// Blocks certain matches so recipient cannot be assigned to giver
//     pub fn block_match(&mut self, recipient: String, giver: String) {
//         self.block
//             .entry(recipient)
//             .or_insert(Vec::new())
//             .push(giver);
//     }

//     /// Parses a line of SecretSanta instructions
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// let mut santa = SecretSanta::new();
//     /// ```
//     ///
//     pub fn parse_instruction(&mut self, instruction: String) -> Result<(), String> {
//         let re_comments = Regex::new(r"(?<instr>[^#]+)+#?.*").unwrap();
//         let Some(caps) = re_comments.captures(instruction.as_str()) else {
//             // whole line is a comment or empty
//             return Err(format!("No valid instruction in line: '{instruction}'"));
//         };
//         let mut instructions = caps["instr"].split_whitespace();
//         let recipient = instructions.next().unwrap();
//         self.add_name(recipient.into());

//         for instr in instructions {
//             panic!("No instructions yet");
//         }

//         return Ok(());
//     }
// }

// #[wasm_bindgen_test]
#[cfg(test)]
mod tests {

    use crate::get_secret_santas;

    use super::SecretSanta;
    use rstest::rstest;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test;

    #[test]
    fn test_get_secret_santa() {
        let instructions = "Amy\nTom !Amy\nBen =Amy\n";

        let pairings = get_secret_santas(instructions.to_string());
        assert_eq!(pairings.unwrap(), JsValue::from_str("AAA"))
    }
}
//     use super::SecretSanta;
//     use rstest::rstest;
//     use wasm_bindgen::JsValue;
//     use wasm_bindgen_test;

//     fn add_names() -> SecretSanta {
//         let mut santa = SecretSanta::new();
//         santa.add_name(String::from("Amy")).unwrap();
//         santa.add_name(String::from("Tom")).unwrap();
//         return santa;
//     }

//     #[test]
//     fn has_names() {
//         let santa = add_names();
//         assert_eq!(santa.names.len(), 2);
//     }

//     fn has_name_already() {
//         let mut santa = add_names();
//         let result = santa.add_name(String::from("Amy"));
//         let a = 1;
//         assert_eq!(santa.names.len(), 2);
//         //let expected = Err(JsValue);
//     }

//     fn add_enforcement() -> SecretSanta {
//         let mut santa: SecretSanta = SecretSanta::new();
//         santa.enforce_match(String::from("Amy"), String::from("Tom"));
//         return santa;
//     }

//     #[test]
//     fn has_enforcement() {
//         let santa = add_enforcement();
//         let key = String::from("Amy");
//         let value = santa.enforce.get(&key);
//         assert_eq!(&String::from("Tom"), value.unwrap());
//     }

//     fn add_blocks() -> SecretSanta {
//         let mut santa: SecretSanta = SecretSanta::new();
//         santa.block_match(String::from("Amy"), String::from("Tom"));
//         santa.block_match(String::from("Amy"), String::from("Ben"));
//         return santa;
//     }

//     #[test]
//     fn has_blocks() {
//         let santa = add_blocks();
//         let amy_blocks = santa.block.get("Amy").unwrap();
//         assert_eq!(amy_blocks.len(), 2);
//         assert!(amy_blocks.iter().any(|e| e == "Tom"));
//         assert!(amy_blocks.iter().any(|e| e == "Ben"));
//     }

//     #[rstest]
//     #[case("Amy".into())]
//     fn can_parse_instructions(#[case] line: String) {
//         let mut santa = SecretSanta::new();
//         santa.parse_instruction(line).unwrap();
//     }
// }

//     fn get_index(&self, row: u32, column: u32) -> usize {
//         (row * self.width + column) as usize
//     }

//     fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
//         let mut count = 0;
//         for delta_row in [self.height - 1, 0, 1].iter().cloned() {
//             for delta_col in [self.width - 1, 0, 1].iter().cloned() {
//                 if delta_row == 0 && delta_col == 0 {
//                     // skip self
//                     continue;
//                 }

//                 let neighbour_row = (row + delta_row) % self.height;
//                 let neighbour_col = (column + delta_col) % self.width;
//                 let idx = self.get_index(neighbour_row, neighbour_col);
//                 count += self.cells[idx] as u8;
//             }
//         }
//         return count;
//     }

//     pub fn tick(&mut self) {
//         let mut next = self.cells.clone();

//         for row in 0..self.height {
//             for col in 0..self.width {
//                 let idx = self.get_index(row, col);
//                 let cell = self.cells[idx];
//                 let live_neighbours = self.live_neighbour_count(row, col);

//                 let next_cell = match (cell, live_neighbours) {
//                     // Rule 1: Any live cell with few than two neighbours dies.
//                     (Cell::Alive, x) if x < 2 => Cell::Dead,
//                     // Rule 2: Any live cell with two or three live neighbours lives.
//                     (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
//                     // Rule 3: Any live cell with more than three live neighbours dies.
//                     (Cell::Alive, x) if x > 3 => Cell::Dead,
//                     // Rule 4: Any dead cell with exactly three live neighbours is live.
//                     (Cell::Dead, 3) => Cell::Alive,
//                     // All other cells unchanged
//                     (otherwise, _) => otherwise,
//                 };

//                 next[idx] = next_cell;
//             }
//         }
//         self.cells = next;
//     }

//     pub fn new() -> Universe {
//         let width = 64;
//         let height = 64;

//         let cells = (0..width * height)
//             .map(|i| {
//                 if i % 2 == 0 || i % 7 == 0 {
//                     Cell::Alive
//                 } else {
//                     Cell::Dead
//                 }
//             })
//             .collect();

//         return Universe {
//             width,
//             height,
//             cells,
//         };
//     }

//     pub fn render(&self) -> String {
//         self.to_string()
//     }

//     pub fn width(&self) -> u32 {
//         self.width
//     }

//     pub fn height(&self) -> u32 {
//         self.height
//     }

//     pub fn cells(&self) -> *const Cell {
//         self.cells.as_ptr()
//     }
// }

// impl fmt::Display for Universe {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
//                 write!(f, "{}", symbol)?;
//             }
//             write!(f, "\n")?;
//         }

//         Ok(())
//     }
// }
