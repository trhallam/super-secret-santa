use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::EncodeUtf16;

use super::error::SecretSantaError;

#[derive(Default, Debug)]
pub struct Participant {
    pub name: String,
    pub paired_with: Option<String>,
    pub blocklist: Option<HashSet<String>>,
}

impl PartialEq for Participant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Participant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// Removes comments starting with # from a line of instruction
fn get_instruction(instruction: &str) -> Option<&str> {
    let re = Regex::new(r"(^[^#]+)").unwrap();
    let Some((_, [instr])) = re.captures(&instruction).map(|cap| cap.extract()) else {
        return None;
    };
    Some(instr.trim())
}

/// Get the name of the recipient
fn parse_recipient(instruction: &str) -> Result<&str, SecretSantaError> {
    let re = Regex::new(r"(^[^=!#]+)").unwrap();
    let Some((_, [recipient])) = re.captures(instruction.trim()).map(|cap| cap.extract()) else {
        return Err(SecretSantaError::new(format!(
            "Could not determine recipient from: {}",
            instruction
        )));
    };
    // remove surrounding white space
    Ok(recipient.trim())
}

/// Get the forced pairing for recipient, they will be the SecretSanta for this person.
fn parse_forced_pairing(instruction: &str) -> Option<&str> {
    let re = Regex::new(r"(=[^=!#]+)").unwrap();
    let (_, [giver]) = re.captures(instruction.trim()).map(|cap| cap.extract())?;

    // remove surrounding white space and prfix
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

    // recipient err if none or if duplicate
    let recipient = parse_recipient(clean_instr)?.to_string();

    // blocklist
    let blocklist: Option<HashSet<String>> =
        parse_blocks(clean_instr).map(|m| m.into_iter().map(|n| n.to_string()).collect());

    // enforced matches
    let paired_with = parse_forced_pairing(&clean_instr).map(|m| m.to_string());

    // return
    Ok(Participant {
        name: recipient,
        paired_with: paired_with,
        blocklist: blocklist,
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::rstest;

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
    fn test_parse_recipient_ok(#[case] line: &str, #[case] exp: &str) {
        let recipient = parse_recipient(&line);
        assert_eq!(exp, recipient.unwrap());
    }

    #[rstest]
    #[case("#comments only")]
    #[case("")]
    #[case("!block only")]
    #[case("=force !block name")]
    fn test_parse_recipient_err(#[case] line: &str) {
        let res = parse_recipient(&line);
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
