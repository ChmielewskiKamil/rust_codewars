fn to_camel_case(text: &str) -> String {
    let mut capitalize_next = false;

    text.chars()
        .map(|c| {
            let camel_cased_result = if capitalize_next {
                c.to_uppercase().next().unwrap()
            } else {
                c
            };

            match c {
                '-' | '_' => capitalize_next = true,
                _ => capitalize_next = false,
            }

            camel_cased_result
        })
        .filter(|c| *c != '-' && *c != '_')
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
    fn it_does_not_modify_input_if_already_camel_case() {
        assert_eq!(to_camel_case("DoNotModifyThat"), "DoNotModifyThat");
    }

    #[test]
    fn it_removes_dashes() {
        assert_eq!(to_camel_case("Remove-Dashes-Here"), "RemoveDashesHere");
    }

    #[test]
    fn it_removes_underscores() {
        assert_eq!(
            to_camel_case("Remove_Underscores_Here"),
            "RemoveUnderscoresHere"
        );
    }

    #[test]
    fn it_capitalizes_letters_after_dash() {
        assert_eq!(
            to_camel_case("It-capitalizes-letters"),
            "ItCapitalizesLetters"
        );
    }

    #[test]
    fn it_capitalizes_letters_after_underscore() {
        assert_eq!(
            to_camel_case("It_capitalizes_letters"),
            "ItCapitalizesLetters"
        );
    }
}
