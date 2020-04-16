pub struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut d = 0;
        let mut r = (0, 0);
        for c in s.chars() {
            match c {
                '(' => d += 1,
                ')' => {
                    if d + r.1 <= 0 {
                        return false;
                    }
                    d -= 1;
                    if d + r.0 < 0 {
                        r.0 = -d;
                        if r.0 > r.1 {
                            return false;
                        }
                    }
                }
                '*' => {
                    if d > 0 {
                        r.0 -= 1;
                    }
                    r.1 += 1;
                }
                _ => {}
            }
        }
        r.0 + d <= 0 && 0 <= r.1 + d
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
