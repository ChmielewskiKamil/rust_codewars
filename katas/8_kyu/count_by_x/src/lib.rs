use std::vec;

/// `x` is the number to be multiplied
/// `n` is how many times `x` will be multiplied
fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut v: Vec<u32> = vec![];
    for i in 1..n + 1 {
        v.push(x * i);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn it_returns_1_for_1_1() {
        assert_eq!(count_by(1, 1), vec![1]);
    }

    #[test]
    fn it_returns_2_elements_for_1_2() {
        assert_eq!(count_by(1, 2), vec![1, 2]);
    }

    #[test]
    fn it_returns_10_elements_for_2_10() {
        assert_eq!(count_by(2, 10), vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
    }
}
