#[allow(dead_code)]
fn get_age(age: &str) -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_2_for_two_years_old() {
        assert_eq!(get_age("2 years old"), 2);
    }

    #[test]
    fn it_returns_3_for_three_years_old() {
        assert_eq!(get_age("3 years old"), 3);
    }
}
