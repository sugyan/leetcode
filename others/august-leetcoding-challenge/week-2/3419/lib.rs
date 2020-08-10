pub struct Solution {}

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .fold(0, |acc, x| acc * 26 + (x - b'A') as i32 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::title_to_number("A".to_string()));
    }

    #[test]
    fn example_2() {
        assert_eq!(28, Solution::title_to_number("AB".to_string()));
    }

    #[test]
    fn example_3() {
        assert_eq!(701, Solution::title_to_number("ZY".to_string()));
    }
}
