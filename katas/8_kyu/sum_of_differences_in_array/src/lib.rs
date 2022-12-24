#![allow(dead_code, unused_variables, unused_imports)]

fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.len() == 1 || arr.is_empty() {
        None
    } else {
        let mut sorted_array = arr.to_owned();
        sorted_array.sort_by(|a, b| b.cmp(a));

        let mut sum = 0;
        for i in 1..sorted_array.len() {
            sum += sorted_array[i - 1] - sorted_array[i];
        }
        Some(sum)
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
    fn it_returns_none_if_only_one_element() {
        assert_eq!(sum_of_differences(&[1]), None);
    }

    #[test]
    fn it_returns_none_for_empty_array() {
        assert_eq!(sum_of_differences(&[]), None);
    }

    #[test]
    fn it_returns_one_for_2_and_1() {
        assert_eq!(sum_of_differences(&[2, 1]), Some(1));
    }

    #[test]
    fn it_returns_three_for_5_and_2() {
        assert_eq!(sum_of_differences(&[5, 2]), Some(3));
    }

    #[test]
    fn it_should_not_depend_on_the_order_of_elemetns() {
        assert_eq!(sum_of_differences(&[2, 5]), Some(3));
    }

    #[test]
    fn it_works_for_two_negative_numbers() {
        assert_eq!(sum_of_differences(&[-2, -3]), Some(1));
    }

    #[test]
    fn it_works_for_three_positive_numbers() {
        assert_eq!(sum_of_differences(&[1, 2, 3]), Some(2));
    }
}
