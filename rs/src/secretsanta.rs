use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

use crate::participant;

use super::error::SecretSantaError;
use super::parse_instruction;
use super::participant::Participant;

pub struct SecretSanta {
    participants: Vec<Participant>,
    recipients: HashSet<String>,
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
        let participants = Vec::new();
        let recipients = HashSet::new();
        return SecretSanta {
            participants,
            recipients,
        };
    }

    /// SecretSanta contains recipient with name
    pub fn contains(&self, name: &str) -> bool {
        self.participants.iter().any(|p| &p.name == name)
    }

    /// Add an instruction to the SecretSanta (e.g. a Recipient with some or no restrictions)
    pub fn add_instruction(&mut self, instruction: &str) -> Result<(), SecretSantaError> {
        let recipient = parse_instruction(&instruction)?;

        // cannot add the same recipient twice
        if self.contains(&recipient.name) {
            let msg = format!("recipient {} already exists", recipient.name);
            return Err(SecretSantaError::new(msg));
        }

        self.participants.push(recipient);
        Ok(())
    }

    /// Update the available recipients
    fn update_recipients(&mut self) -> Result<(), SecretSantaError> {
        self.recipients = self.participants.iter().map(|p| p.name.clone()).collect();

        Ok(())
    }

    fn _gen_pairings(
        &mut self,
        participants: &mut HashSet<String>,
        recipients: &mut HashSet<String>,
    ) -> Result<(), SecretSantaError> {
        // recursive find pairings by eliminating options

        // see if a participant has a single match yet
        for p in self.participants.iter() {

            // sp = self.participants.get(p).unwrap();
            // if recipients.len() == 1 && sp.blocklist.is_none();
            // let possible_matches = self.participants.bl
            // if self.participants.get(p).is_some() {
            //     recipients.insert(p.clone());
            //     participants.remove(p);
            // }
        }

        // check for unitary pairings
        for p in self.participants.iter() {}

        Ok(())
    }

    /// Generate all the SecretSanta pairing.
    pub fn gen_pairing(&mut self) -> Result<(), SecretSantaError> {
        // These two HashSets will be slowly consumed as we assign pairings
        // create a list of all participants people who need a recipient
        let mut participants: HashSet<&mut Participant> = self.participants.iter_mut().collect();
        // let mut participants: HashSet<String> =
        //     self.participants.iter().map(|p| p.name.clone()).collect();
        // create a list of all recipients people who will receive a gift
        let mut recipients = participants.clone();

        // remove participants who have a recipient and recipients who have a participant
        for p in self.participants.iter() {
            if p.paired_with.is_some() {
                participants.remove(p.name.as_str());
                recipients.remove(p.paired_with.as_ref().unwrap().as_str());
            }
        }

        self._gen_pairings(&mut participants, &mut recipients)?;
        // for r in self.recipients.iter() {
        //     // remove participants who have a pairing already
        //     if r.paired_with.is_some() {
        //         let pairing = r.paired_with.as_ref().unwrap();
        //         available_recipients.remove(pairing.as_str());
        //     } else {
        //     }
        // }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Amy", true)]
    #[case("Bob # with comments", false)]
    fn test_contains(#[case] line: &str, #[case] exp: bool) {
        let mut ssanta = SecretSanta::new();
        let res = ssanta.add_instruction(&line);
        assert_eq!(exp, ssanta.contains("Amy"));
    }

    // #[rstest]
    // fn can_parse_instructions() {
    //     let amy = "Amy".to_string();
    //     let tom = "Tom".to_string();
    //     let ben = "Ben".to_string();

    //     let mut santa = SecretSanta::new();
    //     santa.parse_instruction("Amy".to_string()).unwrap();
    //     assert!(santa.names.contains(&amy));

    //     // blocks
    //     santa.parse_instruction("Tom !Amy".to_string()).unwrap();
    //     assert!(santa.names.contains(&tom));
    //     assert!(santa.block.contains_key(&tom));

    //     // enforce
    //     santa
    //         .parse_instruction("Ben =Amy #Comment  ".to_string())
    //         .unwrap();
    //     assert!(santa.names.contains(&ben));
    //     assert!(santa.enforce.contains_key(&ben))
    // }
}
