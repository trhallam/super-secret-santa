use regex::Regex;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use super::error::SecretSantaError;

#[derive(Default, Debug, Eq, Clone)]
pub struct Participant {
    pub name: String,
    pub paired_with: Option<String>,
    pub blocklist: Option<HashSet<String>>,
}

impl PartialEq for Participant {
    // Only need name to match as name is the unique id
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq<Participant> for String {
    fn eq(&self, other: &Participant) -> bool {
        self == &other.name
    }
}

impl PartialEq<String> for Participant {
    fn eq(&self, other: &String) -> bool {
        &self.name == other
    }
}

impl Hash for Participant {
    // Only need name in hash as it is the unique bit
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Participant {
    /// Returns a set of possible matches based on conditions
    pub fn find_matches(&self, names: &HashSet<String>) -> HashSet<String> {
        // Create a set to store filtered results
        let mut matches: HashSet<String> = HashSet::new();

        for p in names {
            // check if the p is not equal to self or in blocklist
            if &self.name != p && !self.blocklist.as_ref().map_or(false, |bl| bl.contains(p)) {
                matches.insert(p.clone());
            }
        }
        matches
    }

    pub fn new(name: String) -> Self {
        Participant {
            name,
            blocklist: None,   // Default to None
            paired_with: None, // Default to None
        }
    }

    pub fn set_paired_with(&mut self, pairing: Option<String>) {
        self.paired_with = pairing;
    }
}

/// Removes comments starting with # from a line of instruction
fn get_instruction(instruction: &str) -> Option<&str> {
    let re = Regex::new(r"(^[^#]+)").unwrap();
    let Some((_, [inst])) = re.captures(&instruction).map(|cap| cap.extract()) else {
        return None;
    };
    Some(inst.trim())
}

/// Get the name of the participant
fn parse_participant(instruction: &str) -> Result<&str, SecretSantaError> {
    let re = Regex::new(r"(^[^=!#]+)").unwrap();
    let Some((_, [participant])) = re.captures(instruction.trim()).map(|cap| cap.extract()) else {
        return Err(SecretSantaError::new(format!(
            "Could not determine participant from: {}",
            instruction
        )));
    };
    // remove surrounding white space
    Ok(participant.trim())
}

/// Get the forced pairing for participant, they will be the SecretSanta for this person.
fn parse_forced_pairing(instruction: &str) -> Option<&str> {
    let re = Regex::new(r"(=[^=!#]+)").unwrap();
    let (_, [giver]) = re.captures(instruction.trim()).map(|cap| cap.extract())?;

    // remove surrounding white space and prefix
    Some(giver.trim().strip_prefix("=").unwrap())
}

/// Get givers blocked from getting this person.
fn parse_blocks(instruction: &str) -> Option<HashSet<&str>> {
    let re = Regex::new(r"(![^=!#]+)").unwrap();
    let mut blocks: HashSet<&str> = HashSet::new();
    for (_, [blk]) in re.captures_iter(&instruction).map(|c| c.extract()) {
        let b = blk.trim().strip_prefix("!").unwrap();
        blocks.insert(b);
    }
    // Check empty
    match blocks.is_empty() {
        true => None,
        false => Some(blocks),
    }
}

/// Parses a line of SecretSanta instructions
///
/// # Examples
///
/// ```
/// use super_secret_santa::parse_instruction;
/// // add a simple person
///
/// let r1 = parse_instruction("Amy");
///
/// // add a person who cannot get giver Tom
/// let r2 = parse_instruction("Ben !Tom");
///
/// // add a person but put in a comment(ignored)
/// let r3 = parse_instruction("Joy # what a joy");
///
/// // ensure a person is matched with a specific giver
/// let r4 = parse_instruction("Molly =Joy");
/// ```
///
pub fn parse_instruction(instruction: &str) -> Result<Participant, SecretSantaError> {
    // remove comments
    let clean_instr = get_instruction(&instruction).unwrap_or("");

    // participant err if none or if duplicate
    let participant = parse_participant(clean_instr)?.to_string();

    // blocklist
    let blocklist: Option<HashSet<String>> =
        parse_blocks(clean_instr).map(|m| m.into_iter().map(|n| n.to_string()).collect());

    // enforced matches
    let paired_with = parse_forced_pairing(&clean_instr).map(|m| m.to_string());

    // return
    Ok(Participant {
        name: participant,
        paired_with: paired_with,
        blocklist: blocklist,
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::{fixture, rstest};

    #[fixture]
    fn amy() -> Participant {
        Participant {
            name: String::from("Amy"),
            paired_with: None,
            blocklist: None,
        }
    }

    #[fixture]
    fn ben() -> Participant {
        Participant {
            name: String::from("Ben"),
            paired_with: None,
            blocklist: Some(HashSet::from(["Amy".to_string()])),
        }
    }

    #[fixture]
    fn tom() -> Participant {
        Participant {
            name: String::from("Tom"),
            paired_with: None,
            blocklist: Some(HashSet::from(["Amy".to_string(), "Ben".to_string()])),
        }
    }

    #[fixture]
    fn participants(amy: Participant, ben: Participant, tom: Participant) -> HashSet<Participant> {
        HashSet::from([amy, ben, tom])
    }

    #[rstest]
    fn test_struct_participant_eq(amy: Participant) {
        let other_amy = amy.clone();
        assert_eq!(amy, other_amy);
        assert_eq!(amy, "Amy".to_string());
        assert_eq!("Amy".to_string(), amy);
    }

    #[rstest]
    fn test_struct_participant_ne(amy: Participant, ben: Participant) {
        assert_ne!(amy, ben);
        assert_ne!(amy, "Ben".to_string());
        assert_ne!("Ben".to_string(), amy);
    }

    #[rstest]
    fn test_hash(amy: Participant, ben: Participant) {
        let hs = HashSet::from([amy.clone()]);
        assert!(hs.contains(&amy));
        assert!(!hs.contains(&ben))
    }

    #[rstest]
    fn test_find_matches(amy: Participant, ben: Participant, tom: Participant) {
        let participants: HashSet<String> =
            HashSet::from([amy.name.clone(), ben.name.clone(), tom.name.clone()]);
        let m1 = amy.find_matches(&participants);
        assert_eq!(m1.len(), 2);
        let m2 = ben.find_matches(&participants);
        assert_eq!(m2.len(), 1);
        let m3 = tom.find_matches(&participants);
        assert_eq!(m3.len(), 0);
    }

    #[rstest]
    #[case("Amy", "Amy")]
    #[case("Amy # with comments", "Amy")]
    #[case("Amy # with comments", "Amy")]
    #[case(" Amy =Tom !Ben ### with comments", "Amy =Tom !Ben")]
    #[case("#comments only", "")]
    #[case("  # indented comments only", "")]
    fn test_get_instruction(#[case] line: &str, #[case] exp: &str) {
        let res = get_instruction(&line);
        assert_eq!(exp, res.unwrap_or(""));
    }

    #[rstest]
    #[case("Amy", "Amy")]
    #[case("Amy ", "Amy")]
    #[case("Amy !Ben ", "Amy")]
    #[case(" Amy =Tom !Ben", "Amy")]
    fn test_parse_participant_ok(#[case] line: &str, #[case] exp: &str) {
        let participant = parse_participant(&line);
        assert_eq!(exp, participant.unwrap());
    }

    #[rstest]
    #[case("#comments only")]
    #[case("")]
    #[case("!block only")]
    #[case("=force !block name")]
    fn test_parse_participant_err(#[case] line: &str) {
        let res = parse_participant(&line);
        assert!(res.is_err());
    }

    #[rstest]
    #[case("!block only", vec!["block only"])]
    #[case("Amy !Tom !Ben =Ron # comment", vec!["Tom", "Ben"])]
    #[case("Amy !Tom !Ben#immediate comment", vec!["Tom", "Ben"])]
    fn test_parse_blocks_ok(#[case] line: &str, #[case] blocks: Vec<&str>) {
        let res = parse_blocks(&line);
        let exp: HashSet<&str> = blocks.iter().cloned().map(|b| b).collect();
        assert_eq!(exp, res.unwrap());
    }

    #[rstest]
    #[case("Amy")]
    #[case("Amy =Ben #Comment")]
    fn test_parse_blocks_none(#[case] line: &str) {
        assert!(parse_blocks(&line).is_none())
    }

    #[rstest]
    #[case("Amy =Ben", "Ben")]
    #[case("Amy =Ben =Tom", "Ben")]
    #[case("Amy =Ben", "Ben")]
    #[case("Amy !Tom =Ben", "Ben")]
    fn test_parse_enforce_ok(#[case] line: &str, #[case] pair: &str) {
        let res = parse_forced_pairing(&line);
        assert_eq!(pair, res.unwrap())
    }

    #[rstest]
    #[case("Amy  ")]
    #[case("Amy #with comment")]
    #[case("Amy !blocks")]
    fn test_parse_enforce_none(#[case] line: &str) {
        let res = parse_forced_pairing(&line);
        assert!(res.is_none());
    }

    #[rstest]
    fn can_parse_instructions() {
        let amy = "Amy".to_string();
        let tom = "Tom".to_string();
        let ben = "Ben".to_string();

        let r1 = parse_instruction("Amy").unwrap();
        assert_eq!(r1.name, amy.to_string());

        // blocks
        let r2 = parse_instruction("Tom !Amy").unwrap();
        assert_eq!(r2.name, tom.to_string());
        assert!(r2.blocklist.unwrap().contains(&amy));

        // enforce
        let r3 = parse_instruction("Ben =Amy #Comment  ").unwrap();
        assert_eq!(r3.name, ben.to_string());
        assert_eq!(r3.paired_with.unwrap(), amy);

        // multiblock skip commented
        let r4 = parse_instruction("Ben !Amy  !Tom #!Sean").unwrap();
        let bl = r4.blocklist.unwrap();
        assert_eq!(r4.name, ben.to_string());
        assert!(r4.paired_with.is_none());
        assert!(bl.contains(&tom) && bl.contains(&amy));
        assert!(!bl.contains("Sean"));
    }
}
