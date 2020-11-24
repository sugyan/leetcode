pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut v: Vec<i32> = Vec::new();
        let mut n = 0;
        let mut prev: u8 = if s.trim_start().starts_with('-') {
            b'-'
        } else {
            b'+'
        };
        for &b in (s + "*1").as_bytes() {
            if b == b' ' {
                continue;
            }
            match b {
                b'0'..=b'9' => {
                    n = n * 10 + (b - b'0') as i32;
                }
                _ => {
                    match prev {
                        b'+' => v.push(n),
                        b'-' => v.push(-n),
                        b'*' => *v.last_mut().unwrap() *= n,
                        b'/' => *v.last_mut().unwrap() /= n,
                        _ => {}
                    }
                    prev = b;
                    n = 0;
                }
            }
        }
        v.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(7, Solution::calculate(String::from("3+2*2")));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::calculate(String::from(" 3/2 ")));
    }

    #[test]
    fn example_3() {
        assert_eq!(5, Solution::calculate(String::from(" 3+5 / 2 ")));
    }
}
