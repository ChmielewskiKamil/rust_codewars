fn find_next_square(sq: u64) -> Option<u64> {
    let mut square_to_test = sq + 1;
    while true {
        if is_perfect(square_to_test) {
            // found = true;
            return Some(square_to_test);
        } else {
            square_to_test += 1;
        }
    }
    None
}

fn is_perfect(sq: u64) -> bool {
    let sqrt = (sq as f64).sqrt();
    let uint_sqrt = sqrt as u64;

    if uint_sqrt * uint_sqrt == sq {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_always_passes() {
        assert!(true);
    }

    /* //////////////////////////////////////////
                find_next_square tests
       /////////////////////////////////////////
    */

    #[test]
    fn it_finds_4_for_2() {
        assert_eq!(find_next_square(2), Some(4));
    }

    #[test]
    fn it_finds_9_for_4() {
        assert_eq!(find_next_square(4), Some(9));
    }

    /* //////////////////////////////////////////
                is_perfect tests
       /////////////////////////////////////////
    */

    #[test]
    fn it_knows_if_4_is_perfect() {
        assert_eq!(is_perfect(4), true);
    }

    #[test]
    fn it_knows_if_5_is_perfect() {
        assert_eq!(is_perfect(5), false);
    }

    #[test]
    fn it_knows_if_121_is_perfect() {
        assert_eq!(is_perfect(121), true);
    }
}
