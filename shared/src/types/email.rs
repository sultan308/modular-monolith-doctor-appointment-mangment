use std::fmt::Display;
use regex::Regex;
use crate::errors::{ApplicationError, ApplicationResult};
const EMAIL_REGEX_STR: &str =  r#"(?i)^(?:[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@[a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?)*$"#;

pub struct Email {
    address: String,
}
impl Email{
    fn validate_address(address: &str) -> ApplicationResult<()>{
        let email_regex = Regex::new(EMAIL_REGEX_STR).unwrap();
        if email_regex.is_match(address) {return Ok(())};
        Err(ApplicationError::InvalidEmail(address.to_string()))
    }
}
impl Email {
    pub fn from(address: String) -> ApplicationResult<Email> {
        Email::validate_address(&address)?;
        Ok(Email{address})
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}