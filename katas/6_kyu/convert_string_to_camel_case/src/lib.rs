use std::iter::Map;

fn to_camel_case(text: &str) -> String {
    if text.contains("-") {
        text.replace("-", "")
    } else if text.contains("_") {
        text.replace("_", "")
    } else {
        text.to_string()
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
}
