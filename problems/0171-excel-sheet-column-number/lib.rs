pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .bytes()
            .fold(0, |acc, x| acc * 26 + (x - b'@') as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::title_to_number(String::from("A")));
    }

    #[test]
    fn example_2() {
        assert_eq!(28, Solution::title_to_number(String::from("AB")));
    }

    #[test]
    fn example_3() {
        assert_eq!(701, Solution::title_to_number(String::from("ZY")));
    }
}
