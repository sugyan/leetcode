pub struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut r = (0, 0);
        for c in s.chars() {
            r.0 += if c == '(' { 1 } else { -1 };
            r.1 += if c != ')' { 1 } else { -1 };
            if r.1 < 0 {
                return false;
            }
            r.0 = std::cmp::max(r.0, 0);
        }
        r.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::check_valid_string("()".to_string()));
    }

    #[test]
    fn example_2() {
        assert_eq!(true, Solution::check_valid_string("(*)".to_string()));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::check_valid_string("(*))".to_string()));
    }
}
