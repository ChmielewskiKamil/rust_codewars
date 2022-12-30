/// Converts a number to a string representating roman numeral.
fn num_as_roman(mut num: i32) -> String {
    let mut roman_number = String::new();
    while num > 0 {
        match num {
            n if n <= 3 => {
                roman_number.push_str("I");
                num -= 1;
            }
            _ => break,
        };
    }
    roman_number
}
#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_always_passes() {
        assert!(true);
    }

    // #[test]
    // fn it_should_fail_if_bigger_than_3999() {
    //     assert_eq!(
    //         num_as_roman(4000),
    //         "Your number: 4000 is to big. Max representable number is 3999!"
    //     )
    // }

    #[test]
    fn it_should_encode_1_to_I() {
        assert_eq!(num_as_roman(1), "I");
    }

    #[test]
    fn it_should_encode_2_to_II() {
        assert_eq!(num_as_roman(2), "II");
    }
}
