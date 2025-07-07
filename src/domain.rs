use std::fmt::Display;

use serde::Deserialize;
use snafu::{Whatever, prelude::*};
use validator::ValidateLength;

#[derive(Debug, Deserialize)]
pub struct SubscriberName(String);

impl Display for SubscriberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl SubscriberName {
    /// Returns an instance of `SubscriberName` if the input satisfies all
    /// our validation constraints on subscriber names.  
    ///
    /// # Errors
    ///
    /// - Supplied String is empty
    /// - Supplied string is <3 or >256 characters
    /// - Contains forbidden characters
    pub fn parse(s: String) -> Result<Self, Whatever> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let name = Self(s);
        let len_valid = name.validate_length(Some(3), Some(256), None);

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = name
            .as_ref()
            .chars()
            .any(|g| forbidden_characters.contains(&g));

        if !len_valid || contains_forbidden_characters || is_empty_or_whitespace {
            whatever!("name is not valid");
        } else {
            Ok(name)
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ValidateLength<u64> for SubscriberName {
    fn length(&self) -> std::option::Option<u64> {
        Some(self.as_ref().chars().count() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberName;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert!(SubscriberName::parse(name).is_ok());
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert!(SubscriberName::parse(name).is_err());
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert!(SubscriberName::parse(name).is_err());
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = String::new();
        assert!(SubscriberName::parse(name).is_err());
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = format!("{name}XXXX");
            assert!(SubscriberName::parse(name).is_err());
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert!(SubscriberName::parse(name).is_ok());
    }
}
