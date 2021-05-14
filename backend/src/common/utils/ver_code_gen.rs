use crate::config::CONFIG;
use slauth::oath::totp::TOTPBuilder;

pub fn gen_totp_code(secret: String) -> String {
    let top_builder = TOTPBuilder::new();
    let ctx = top_builder
        .secret(secret.as_bytes())
        .period(CONFIG.security.onetime_code_duration as u64)
        .digits(CONFIG.security.onetime_code_length as usize)
        .build();
    ctx.gen()
}

pub fn verify_totp_code(code: &str, secret: String) -> bool {
    let top_builder = TOTPBuilder::new();
    let mut ctx = top_builder
        .secret(secret.as_bytes())
        .period(CONFIG.security.onetime_code_duration as u64)
        .digits(CONFIG.security.onetime_code_length as usize)
        .build();
    ctx.verify(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_code() {
        let code = gen_totp_code("secret".to_string());
        assert_eq!(code.len(), 6)
    }

    #[test]
    fn it_verifies_code_successfully() {
        let code = gen_totp_code("secret".to_string());
        let state = verify_totp_code(code.as_str(), "secret".to_string());
        assert_eq!(state, true)
    }

    #[test]
    fn it_fails_to_verify_code() {
        let state = verify_totp_code("123456", "secret".to_string());
        assert_eq!(state, false)
    }
}
