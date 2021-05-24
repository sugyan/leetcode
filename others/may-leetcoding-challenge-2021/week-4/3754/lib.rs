pub struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.as_bytes()
            .iter()
            .map(|u| (u + if (b'A'..=b'Z').contains(u) { 0x20 } else { 0 }) as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("hello", Solution::to_lower_case(String::from("Hello")));
    }

    #[test]
    fn example_2() {
        assert_eq!("here", Solution::to_lower_case(String::from("here")));
    }

    #[test]
    fn example_3() {
        assert_eq!("lovely", Solution::to_lower_case(String::from("LOVELY")));
    }
}
