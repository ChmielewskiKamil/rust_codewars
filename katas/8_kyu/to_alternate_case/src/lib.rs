#[allow(dead_code)]
fn to_alternating_case(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            c if c.is_lowercase() => c.to_uppercase().next().unwrap(),
            c if c.is_uppercase() => c.to_lowercase().next().unwrap(),
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_always_passes() {
        assert!(true);
    }

    #[test]
    fn it_should_change_single_char_from_lower_to_upper() {
        assert_eq!(to_alternating_case("a"), "A");
    }

    #[test]
    fn it_should_change_single_char_from_upper_to_lower() {
        assert_eq!(to_alternating_case("A"), "a");
    }

    #[test]
    fn it_should_change_multiple_chars_from_lower_to_upper() {
        assert_eq!(to_alternating_case("aaa"), "AAA");
    }

    #[test]
    fn it_should_alter_variable_cased_chars() {
        assert_eq!(to_alternating_case("aAa"), "AaA");
    }

    #[test]
    fn it_should_not_alter_numbers() {
        assert_eq!(to_alternating_case("12345"), "12345");
    }

    #[test]
    fn it_should_alter_mixed_chars_not_numbers() {
        assert_eq!(to_alternating_case("12ab34CD56ef"), "12AB34cd56EF");
    }
}
