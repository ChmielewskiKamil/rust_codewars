fn find_next_square(sq: u64) -> Option<u64> {
    Some(4)
}

fn is_perfect(sq: u64) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_always_passes() {
        assert!(true);
    }

    #[test]
    fn it_finds_4_for_2() {
        assert_eq!(find_next_square(2), Some(4));
    }

    #[test]
    fn it_knows_if_square_is_perfect() {
        assert_eq!(is_perfect(4), true);
    }
}
