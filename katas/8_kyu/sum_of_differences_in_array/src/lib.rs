#![allow(dead_code, unused_variables, unused_imports)]

fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_always_passes() {
        assert!(true);
    }

    #[test]
    fn it_returns_none_if_only_one_element() {
        assert_eq!(sum_of_differences(&[1]), None);
    }
}
