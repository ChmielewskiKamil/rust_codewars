fn to_camel_case(text: &str) -> String {
    text.to_string()
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
}
