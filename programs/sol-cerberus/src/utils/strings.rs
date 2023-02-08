use crate::Errors;
use anchor_lang::prelude::*;

pub fn validate_string_len(text: &String, min: usize, max: usize) -> Result<String> {
    if text.as_bytes().len() < min {
        return err!(Errors::StringTooShort);
    }
    if text.as_bytes().len() > max {
        return err!(Errors::StringTooLong);
    }
    Ok(text.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rules() {
        let mut text = "".to_string();
        assert_eq!(validate_string_len(&text, 0, 5), Ok("".to_string()));
        assert_eq!(
            validate_string_len(&text, 1, 5),
            err!(Errors::StringTooShort)
        );
        text = "ABC".to_string();
        assert_eq!(
            validate_string_len(&text, 1, 2),
            err!(Errors::StringTooLong)
        );
    }
}
