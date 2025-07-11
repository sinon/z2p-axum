use std::fmt::Display;

use serde::Deserialize;
use snafu::{Whatever, whatever};
use validator::ValidateEmail;

#[derive(Debug, Deserialize)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    /// Returns an instance of `SubscriberEmail` if the input satisfies all
    /// our validation constraints on subscriber emails.  
    ///
    /// # Errors
    ///
    /// - Supplied email does not conform to RFC 5322
    pub fn parse(input: String) -> Result<Self, Whatever> {
        if input.validate_email() {
            Ok(Self(input))
        } else {
            whatever!("{input} is not a valid subscriber email.")
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, faker::internet::en::SafeEmail, rand::rngs::StdRng};
    use rand::SeedableRng;

    use super::SubscriberEmail;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }

    #[test]
    fn email_valid() {
        let email = "ursula@domain.com".to_string();
        assert!(SubscriberEmail::parse(email).is_ok());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
