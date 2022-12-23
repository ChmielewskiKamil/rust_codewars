#[allow(dead_code)]
fn plural(n: f64) -> bool {
    if n == 1.0 {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_false_for_one() {
        assert_eq!(plural(1.0), false);
    }

    #[test]
    fn it_returns_true_for_two() {
        assert_eq!(plural(2.0), true);
    }

    #[test]
    fn it_returns_true_for_zero() {
        assert_eq!(plural(0), true);
    }
}
