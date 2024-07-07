use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if ValidateEmail::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("Invalid subscriber email: {}", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            ValidEmailFixture(SafeEmail().fake())
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        let result = SubscriberEmail::parse(email);
        assert_err!(result);
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursulaexample.com".to_string();
        let result = SubscriberEmail::parse(email);
        assert_err!(result);
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@example.com".to_string();
        let result = SubscriberEmail::parse(email);
        assert_err!(result);
    }

    #[test]
    fn valid_email_is_accepted() {
        let email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_should_be_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
