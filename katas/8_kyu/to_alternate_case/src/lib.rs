#[allow(dead_code)]
fn to_alternating_case(s: &str) -> String {
    let mut original_string: String = String::new();
    original_string.push_str(s);

    let mut altered_string: String = String::new();

    for character in original_string.chars() {
        if character.is_uppercase() {
            altered_string.push_str(&character.to_lowercase().to_string());
        } else {
            altered_string.push_str(&character.to_uppercase().to_string());
        }
    }
    altered_string
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
}
