use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut str {
        &mut self.0
    }

    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("Invalid subscriber name: {}", s))
        } else {
            Ok(Self(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_long_name_is_not_allowed() {
        let name = "a".repeat(257);
        let result = SubscriberName::parse(name);
        assert_err!(result);
    }

    #[test]
    fn a_256_character_name_is_allowed() {
        let name = "a".repeat(256);
        let result = SubscriberName::parse(name);
        assert_ok!(result);
    }

    #[test]
    fn whitespace_only_name_is_not_allowed() {
        let name = " ".to_string();
        let result = SubscriberName::parse(name);
        assert_err!(result);
    }

    #[test]
    fn empty_name_is_not_allowed() {
        let name = "".to_string();
        let result = SubscriberName::parse(name);
        assert_err!(result);
    }

    #[test]
    fn name_with_forbidden_characters_is_not_allowed() {
        for &name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let result = SubscriberName::parse(name.to_string());
            assert_err!(result);
        }
    }

    #[test]
    fn valid_names_are_parsed_successfully() {
        let name = "Ada Lovelace".to_string();
        let result = SubscriberName::parse(name);
        assert_ok!(result);
    }
}
