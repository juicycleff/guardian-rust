use crate::config::CONFIG;
use std::borrow::Cow;
use std::collections::HashMap;
use validator::ValidationError;
use zxcvbn::zxcvbn;

/// This function takes in a password string and validates them
/// It should be used with the `validator.rs` crate as a derive
#[allow(unused)]
pub fn validate_strong_password(password: &str) -> Result<(), ValidationError> {
    let estimate = zxcvbn(password, &[]).unwrap();

    if estimate.score() < CONFIG.security.password_strength as u8 {
        return Err(ValidationError {
            code: Cow::from("invalid_password"),
            message: Option::from(Cow::from("Password is not strong enough")),
            params: HashMap::new(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_invalidates_weak_password() {
        let password = validate_strong_password("weak_password");
        let expected = Err(ValidationError {
            code: Cow::from("invalid_password"),
            message: Option::from(Cow::from("Password is not strong enough")),
            params: HashMap::new(),
        });

        assert_eq!(expected, password);
    }

    #[test]
    fn it_validates_strong_password() {
        let password = validate_strong_password("@43874hfdfweak_password").unwrap();
        assert_eq!((), password);
    }
}
