/// Converts a number to a string representating roman numeral.
fn num_as_roman(mut num: i32) -> String {
    let mut roman_number = String::new();
    while num > 0 {
        match num {
            n if n >= 5 => {
                roman_number.push_str("V");
                num -= 5;
            }
            n if n >= 4 => {
                roman_number.push_str("IV");
                num -= 4;
            }
            n if n >= 1 => {
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

    #[test]
    fn it_should_encode_4_to_IV() {
        assert_eq!(num_as_roman(4), "IV");
    }

    #[test]
    fn it_should_encode_5_to_V() {
        assert_eq!(num_as_roman(5), "V");
    }
}
