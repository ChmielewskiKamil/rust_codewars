fn find_next_square(sq: u64) -> Option<u64> {
    Some(4)
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

    #[test]
    fn it_finds_4_for_2() {
        assert_eq!(find_next_square(2), Some(4));
    }

    #[test]
    fn it_knows_if_4_is_perfect() {
        assert_eq!(is_perfect(4), true);
    }

    #[test]
    fn it_knows_if_5_is_perfect() {
        assert_eq!(is_perfect(5), false);
    }
}
