/// Converts a number to a string representating roman numeral.
fn num_as_roman(mut num: i32) -> String {
    let mut roman_number = String::new();
    while num > 0 {
        match num {
            n if n >= 1000 => {
                roman_number.push_str("M");
                num -= 1000;
            }
            n if n >= 900 => {
                roman_number.push_str("CM");
                num -= 900;
            }
            n if n >= 500 => {
                roman_number.push_str("D");
                num -= 500;
            }
            n if n >= 400 => {
                roman_number.push_str("CD");
                num -= 400;
            }
            n if n >= 100 => {
                roman_number.push_str("C");
                num -= 100;
            }
            n if n >= 90 => {
                roman_number.push_str("XC");
                num -= 90;
            }
            n if n >= 50 => {
                roman_number.push_str("L");
                num -= 50;
            }
            n if n >= 40 => {
                roman_number.push_str("XL");
                num -= 40;
            }
            n if n >= 10 => {
                roman_number.push_str("X");
                num -= 10;
            }
            n if n >= 9 => {
                roman_number.push_str("IX");
                num -= 9;
            }
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

    #[test]
    fn it_should_fail_if_bigger_than_3999() {
        assert_eq!(
            num_as_roman(4000),
            "Your number: 4000 is to big. Max representable number is 3999!"
        )
    }

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

    #[test]
    fn it_should_encode_9_to_IX() {
        assert_eq!(num_as_roman(9), "IX");
    }

    #[test]
    fn it_should_encode_10_to_X() {
        assert_eq!(num_as_roman(10), "X");
    }

    #[test]
    fn it_should_encode_11_to_XI() {
        assert_eq!(num_as_roman(11), "XI");
    }

    #[test]
    fn it_should_encode_40_to_XL() {
        assert_eq!(num_as_roman(40), "XL");
    }

    #[test]
    fn it_should_encode_50_to_L() {
        assert_eq!(num_as_roman(50), "L");
    }

    #[test]
    fn it_should_encode_90_to_XC() {
        assert_eq!(num_as_roman(90), "XC");
    }

    #[test]
    fn it_should_encode_100_to_C() {
        assert_eq!(num_as_roman(100), "C");
    }

    #[test]
    fn it_should_encode_400_to_CD() {
        assert_eq!(num_as_roman(400), "CD");
    }

    #[test]
    fn it_should_encode_500_to_D() {
        assert_eq!(num_as_roman(500), "D");
    }

    #[test]
    fn it_should_encode_900_to_CM() {
        assert_eq!(num_as_roman(900), "CM");
    }
    #[test]
    fn it_should_encode_1000_to_M() {
        assert_eq!(num_as_roman(1000), "M");
    }
}
