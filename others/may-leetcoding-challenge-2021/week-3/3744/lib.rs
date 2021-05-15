pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        if let Some(e) = s.chars().position(|c| c == 'e' || c == 'E') {
            Self::is_decimal(&s[0..e]) && Self::is_integer(&s[e + 1..])
        } else {
            Self::is_decimal(&s)
        }
    }
    fn is_decimal(s: &str) -> bool {
        if Self::is_integer(s) {
            return true;
        }
        let (mut num, mut dot) = (false, false);
        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' => num = true,
                '+' | '-' if i == 0 => {}
                '.' if !dot => dot = true,
                _ => return false,
            }
        }
        num
    }
    fn is_integer(s: &str) -> bool {
        let mut num = false;
        for (i, c) in s.chars().enumerate() {
            match c {
                '0'..='9' => num = true,
                '+' | '-' if i == 0 => {}
                _ => return false,
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert!([
            "2",
            "0089",
            "-0.1",
            "+3.14",
            "4.",
            "-.9",
            "2e10",
            "-90E3",
            "3e+7",
            "+6e-1",
            "53.5e93",
            "-123.456e789",
        ]
        .iter()
        .all(|&s| Solution::is_number(s.to_string())));
        assert!(
            ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]
                .iter()
                .all(|&s| !Solution::is_number(s.to_string()))
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_number(String::from("0")));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_number(String::from("e")));
    }

    #[test]
    fn example_3() {
        assert_eq!(false, Solution::is_number(String::from(".")));
    }

    #[test]
    fn example_4() {
        assert_eq!(true, Solution::is_number(String::from(".1")));
    }
}
