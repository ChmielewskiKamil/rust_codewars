fn find_next_square(sq: u64) -> Option<u64> {
    let square_root = (sq as f64).sqrt();
    if square_root.floor() == square_root {
        return Some(((square_root as u64) + 1).pow(2));
    } else {
        None
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
    fn it_returns_none_for_2() {
        assert_eq!(find_next_square(2), None);
    }

    #[test]
    fn it_finds_9_for_4() {
        assert_eq!(find_next_square(4), Some(9));
    }

    #[test]
    fn it_finds_676_for_625() {
        assert_eq!(find_next_square(625), Some(676));
    }

    #[test]
    fn it_returns_none_for_114() {
        assert_eq!(find_next_square(114), None);
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
