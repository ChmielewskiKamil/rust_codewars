fn to_alternating_case(s: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
    #[test]
    fn it_should_change_casing_from_lower_to_upper() {
        assert_eq!(to_alternating_case("a"), "A");
    }
}
