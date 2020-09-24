pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        s.as_bytes()
            .iter()
            .chain(t.as_bytes())
            .fold(0, |acc, &x| acc ^ x) as char
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
}
