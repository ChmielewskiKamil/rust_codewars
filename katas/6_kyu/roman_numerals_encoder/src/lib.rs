/// Converts a number to a string representating roman numeral.
fn num_as_roman(num: i32) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_always_passes() {
        assert!(true);
    }

    #[test]
    fn it_should_fail_if_bigger_than_3999() {
        assert_eq!(
            num_as_roman(4000),
            "Your number: 4000 is to big. Max representable number is 3999!"
        )
    }
}
