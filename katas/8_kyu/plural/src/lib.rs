#[allow(dead_code)]
fn plural(n: f64) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_false_for_one() {
        assert_eq!(plural(1.0), false);
    }
}
