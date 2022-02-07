pub struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        (s + &t).bytes().fold(0, |acc, u| acc ^ u) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            'e',
            Solution::find_the_difference(String::from("abcd"), String::from("abcde"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            'y',
            Solution::find_the_difference(String::from(""), String::from("y"))
        );
    }
}
