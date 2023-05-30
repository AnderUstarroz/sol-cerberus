use crate::Errors;
use anchor_lang::prelude::*;

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

pub fn allowed_perm(rule1: &String, rule2: &String) -> bool {
    if rule1 == rule2 || rule2 == "*" {
        return true;
    }

    false
}

pub fn validate_ns_permission(namespace: &String) -> Result<()> {
    if namespace != &"*" {
        if let Err(_) = namespace.parse::<u8>() {
            return err!(Errors::InvalidNamespace);
        }
    }
    Ok(())
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
    fn test_valid_permission() {
        assert_eq!(allowed_perm(&"add".to_string(), &"add".to_string()), true);
        assert_eq!(allowed_perm(&"add".to_string(), &"edit".to_string()), false);
        assert_eq!(allowed_perm(&"add".to_string(), &"*".to_string()), true);
    }

    #[test]
    fn test_validate_ns_permission() {
        assert_eq!(validate_ns_permission(&"*".to_string()), Ok(()));
        assert_eq!(validate_ns_permission(&"0".to_string()), Ok(()));
        assert_eq!(validate_ns_permission(&"1".to_string()), Ok(()));
        assert_eq!(validate_ns_permission(&"255".to_string()), Ok(()));
        assert_eq!(
            validate_ns_permission(&"256".to_string()),
            err!(Errors::InvalidNamespace)
        );
        assert_eq!(
            validate_ns_permission(&"-1".to_string()),
            err!(Errors::InvalidNamespace)
        );
        assert_eq!(
            validate_ns_permission(&"a".to_string()),
            err!(Errors::InvalidNamespace)
        );
    }
}
