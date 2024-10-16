use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use super::error::SecretSantaError;
use super::participant::{parse_instruction, Participant};

pub struct SecretSanta {
    participants: HashSet<Participant>,
}

impl SecretSanta {
    /// Initialises a SecretSanta
    ///
    /// # Examples
    ///
    /// ```
    /// use super_secret_santa::SecretSanta;
    /// let mut santa = SecretSanta::new();
    /// ```
    pub fn new() -> SecretSanta {
        let participants = HashSet::new();
        return SecretSanta { participants };
    }

    /// SecretSanta contains recipient with name
    pub fn contains(&self, name: &str) -> bool {
        self.participants
            .contains(&Participant::new(name.to_string()))
    }

    /// SecretSanta get participant by name
    pub fn get_name(&self, name: &str) -> Option<&Participant> {
        let p = Participant::new(name.to_string());
        let part = self.participants.get(&p)?;
        Some(part)
    }

    /// Get all participant names
    pub fn names(&self) -> HashSet<String> {
        self.participants.iter().map(|p| p.name.clone()).collect()
    }

    /// Add an instruction to the SecretSanta (e.g. a Recipient with some or no restrictions)
    pub fn add_instruction(&mut self, instruction: &str) -> Result<(), SecretSantaError> {
        let part = parse_instruction(&instruction)?;

        // cannot add the same recipient twice
        if self.contains(&part.name) {
            let msg = format!("recipient {} already exists", part.name);
            return Err(SecretSantaError::new(msg));
        }

        self.participants.insert(part);
        Ok(())
    }

    // /// Update the available recipients
    // fn update_recipients(&mut self) -> Result<(), SecretSantaError> {
    //     self.recipients = self.participants.iter().map(|p| p.name.clone()).collect();

    //     Ok(())
    // }

    fn _generate_pairings(
        &mut self,
        mut set_a: HashSet<String>,
        mut set_b: HashSet<String>,
    ) -> Result<(), SecretSantaError> {
        // Sort set_a by number of available matches lowest to highest
        let mut tree_a = BTreeMap::new();

        for p in set_a.iter() {
            let Some(part) = self.get_name(p) else {
                return Err(SecretSantaError::new("Instructions issue".to_string()));
            };
            let n_matches = part.find_matches(&set_b).len();
            tree_a.insert(n_matches, part);
        }

        match tree_a.pop_first() {
            None => Ok(()), // Found matches for everyone, hooray!
            // The first participant has no matches
            Some((n, part)) if n == 0 => Err(SecretSantaError::new(
                format!("Instructions are too restrictive for {}", part.name).to_string(),
            )),
            // The first participant has one match
            Some((n, part)) if n == 1 => {
                let pairing = part
                    .find_matches(&set_b)
                    .iter()
                    .next()
                    .cloned()
                    .expect("Something went wrong");
                // remove giver and receiver from recursive lists
                set_a.remove(&part.name);
                set_b.remove(&pairing);
                // update the participant
                let mut part_new = part.clone();
                part_new.set_paired_with(Some(pairing));
                _ = self.participants.replace(part_new);
                self._generate_pairings(set_a, set_b)
            }
            // There are multiple options for this giver, get them a random one
            Some((n, part)) if n >= 2 => {
                let matches = part.find_matches(&set_b);
                let vec_matches: Vec<&String> = matches.iter().collect();
                let pairing = vec_matches
                    .choose(&mut thread_rng())
                    .cloned()
                    .expect("Something went wrong!");
                set_a.remove(&part.name);
                set_b.remove(pairing);
                let mut part_new = part.clone();
                part_new.set_paired_with(Some(pairing.clone()));
                _ = self.participants.replace(part_new);
                self._generate_pairings(set_a, set_b)
            }
            _ => Ok(()),
        }
    }

    /// Generate all the SecretSanta pairing.
    pub fn generate_pairings(&mut self) -> Result<(), SecretSantaError> {
        // List of participants still available as givers
        // set_a starts a hash set to enable easy removal at the beginning but later we will change
        // it to a Vec to allow sorting based on number of suitable matches.
        let mut set_a: HashSet<String> = self.names();
        // List of participants still available as receivers
        let mut set_b: HashSet<String> = self.names();
        // Here set_a will give a present to set_b

        // Remove names from both sets that are forced pairings
        for p in self.participants.iter() {
            if let Some(paired_with) = p.paired_with.as_ref() {
                set_a.remove(&p.name);
                set_b.remove(paired_with);
            }
        }

        let _ = self._generate_pairings(set_a, set_b)?;

        Ok(())
    }

    pub fn get_pairings(&self) -> HashMap<String, String> {
        let u = "Undefined".to_string();
        self.participants
            .iter()
            .map(|p| (p.name.clone(), p.paired_with.clone().unwrap_or(u.clone())))
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::{fixture, rstest};

    #[rstest]
    #[case("Amy", true)]
    #[case("Bob # with comments", false)]
    fn test_contains(#[case] line: &str, #[case] exp: bool) {
        let mut ssanta = SecretSanta::new();
        let _ = ssanta.add_instruction(&line).unwrap();
        assert_eq!(exp, ssanta.contains("Amy"));
    }

    #[fixture]
    fn test_secret_santa() -> SecretSanta {
        let mut santa = SecretSanta::new();
        santa.add_instruction("Amy").unwrap();
        santa.add_instruction("Chris").unwrap();
        santa.add_instruction("Kara").unwrap();
        // blocks
        santa.add_instruction("Tom !Amy").unwrap();
        santa.add_instruction("Noel !Tom !Chris").unwrap();
        santa.add_instruction("Gary !Tom").unwrap();
        // enforce
        santa.add_instruction("Ben =Amy #Comment  ").unwrap();
        santa
    }

    #[fixture]
    fn broken_secret_santa() -> SecretSanta {
        let mut santa = SecretSanta::new();
        santa.add_instruction("Amy !Chris !Kara").unwrap();
        santa.add_instruction("Chris !Amy !Kara").unwrap();
        santa.add_instruction("Kara").unwrap();
        santa
    }

    #[rstest]
    fn can_parse_instructions() {
        let amy = "Amy".to_string();
        let tom = "Tom".to_string();
        let ben = "Ben".to_string();

        let mut santa = SecretSanta::new();
        santa.add_instruction("Amy").unwrap();
        assert!(santa.contains(&amy));

        // blocks
        santa.add_instruction("Tom !Amy").unwrap();
        assert!(santa.contains(&tom));

        // enforce
        santa.add_instruction("Ben =Amy #Comment  ").unwrap();
        assert!(santa.contains(&ben));
    }

    #[rstest]
    fn test_generate_pairings(mut test_secret_santa: SecretSanta) {
        test_secret_santa.generate_pairings().unwrap();
        assert!(true)
    }

    #[rstest]
    fn test_broken_generate_pairings(mut broken_secret_santa: SecretSanta) {
        let res = broken_secret_santa.generate_pairings();
        assert!(res.is_err());
    }

    #[rstest]
    fn test_get_pairings(mut test_secret_santa: SecretSanta) {
        test_secret_santa.generate_pairings().unwrap();
        let pairings = test_secret_santa.get_pairings();
        assert_eq!(pairings.len(), 7)
    }
}
