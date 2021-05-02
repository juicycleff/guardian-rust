pub fn verification_code_gen(length: u8) -> String {
    let possible = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut code: String = "".to_string();

    for _ in 0..length {
        let raw_position: f32 = rand::random::<f32>() * possible.len() as f32;
        let pos: usize = raw_position.floor() as usize;
        let c = possible[pos];
        code = format!("{}{}", code.clone(), c);
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_code() {
        let code = verification_code_gen(6);
        assert_eq!(code.len(), 6)
    }
}
