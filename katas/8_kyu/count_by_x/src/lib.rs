fn count_by(x: u32, n: u32) -> Vec<u32> {
    todo!()
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
}
