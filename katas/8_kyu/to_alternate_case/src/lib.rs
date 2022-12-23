#[allow(dead_code)]
fn to_alternating_case(s: &str) -> String {
    let mut original_string = String::new();
    original_string.push_str(s);
    if original_string.chars().nth(0).unwrap().is_uppercase() {
        original_string.to_lowercase()
    } else {
        original_string.to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_always_passes() {
        assert!(true);
    }

    #[test]
    fn it_should_change_casing_from_lower_to_upper() {
        assert_eq!(to_alternating_case("a"), "A");
    }

    #[test]
    fn it_should_change_casing_from_upper_to_lower() {
        assert_eq!(to_alternating_case("A"), "a");
    }
}
