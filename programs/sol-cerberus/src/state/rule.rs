use anchor_lang::prelude::*;

#[account]
pub struct Rule {
    pub app_id: Pubkey,
    pub bump: u8,
    pub role: String,
    pub resource: String,
    pub permission: String,
}

pub fn valid_rule(role: &String, resource: &String, permission: &String) -> bool {
    for item in vec![role, resource, permission] {
        if item.is_empty() || item.as_bytes().len() > 16 {
            return false;
        }
        for char in item.chars() {
            if !char.is_ascii_alphanumeric() {
                // Allow wildcard character "*".
                if char == '*' && item.as_bytes().len() == 1 {
                    continue;
                }
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rule() {
        assert_eq!(
            valid_rule(&"a".to_string(), &"b".to_string(), &"c".to_string()),
            true
        );
        // Empty Role, Resource or Permission are not allowed.
        assert_eq!(
            valid_rule(&"".to_string(), &"b".to_string(), &"c".to_string()),
            false
        );
        // 16 Characters max per Role, Resource or Permission.
        assert_eq!(
            valid_rule(
                &"12345678901234567".to_string(),
                &"b".to_string(),
                &"c".to_string()
            ),
            false
        );
        // Only Alphanumeric chars allowed.
        assert_eq!(
            valid_rule(&"-".to_string(), &"b".to_string(), &"C".to_string()),
            false
        );
        // Allow "*" on all fields
        assert_eq!(
            valid_rule(&"*".to_string(), &"*".to_string(), &"*".to_string()),
            true
        );
    }
}
