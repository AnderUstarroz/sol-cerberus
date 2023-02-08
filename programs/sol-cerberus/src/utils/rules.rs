pub fn valid_rule(text: &String, allow_wildcard: bool) -> bool {
    if text.is_empty() || text.as_bytes().len() > 16 {
        return false;
    }
    for char in text.chars() {
        if !char.is_ascii_alphanumeric() {
            // Allow wildcard character "*" on all fields but Role.
            if allow_wildcard && char == '*' && text.as_bytes().len() == 1 {
                continue;
            }
            return false;
        }
    }

    true
}
pub fn valid_rules(role: &String, resource: &String, permission: &String) -> bool {
    for (index, item) in vec![role, resource, permission].iter().enumerate() {
        if !valid_rule(item, index > 0) {
            return false;
        }
    }
    true
}

pub fn allowed_rule(rule1: &String, rule2: &String) -> bool {
    if rule1 == rule2 || rule2 == "*" {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rules() {
        assert_eq!(
            valid_rules(&"a".to_string(), &"b".to_string(), &"c".to_string()),
            true
        );
        // Empty Role, Resource or Permission are not allowed.
        assert_eq!(
            valid_rules(&"".to_string(), &"b".to_string(), &"c".to_string()),
            false
        );
        // 16 Characters max per Role, Resource or Permission.
        assert_eq!(
            valid_rules(
                &"12345678901234567".to_string(),
                &"b".to_string(),
                &"c".to_string()
            ),
            false
        );
        // Only Alphanumeric chars allowed.
        assert_eq!(
            valid_rules(&"-".to_string(), &"b".to_string(), &"C".to_string()),
            false
        );
        // Allow "*" on all fields but Role.
        assert_eq!(
            valid_rules(&"A".to_string(), &"*".to_string(), &"*".to_string()),
            true
        );
        assert_eq!(
            valid_rules(&"*".to_string(), &"B".to_string(), &"C".to_string()),
            false
        );
    }

    #[test]
    fn test_allowed_rule() {
        assert_eq!(allowed_rule(&"add".to_string(), &"add".to_string()), true);
        assert_eq!(allowed_rule(&"add".to_string(), &"edit".to_string()), false);
        assert_eq!(allowed_rule(&"add".to_string(), &"*".to_string()), true);
    }
}
