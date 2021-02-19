pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut v = s.chars().map(Option::Some).collect::<Vec<_>>();
        {
            let mut depth = 0;
            for (i, &b) in s.as_bytes().iter().enumerate() {
                match b {
                    b')' if depth == 0 => v[i] = None,
                    b')' => depth -= 1,
                    b'(' => depth += 1,
                    _ => {}
                }
            }
        }
        {
            let mut depth = 0;
            for (i, &b) in s.as_bytes().iter().enumerate().rev() {
                match b {
                    b'(' if depth == 0 => v[i] = None,
                    b'(' => depth -= 1,
                    b')' => depth += 1,
                    _ => {}
                }
            }
        }
        v.into_iter().filter_map(|e| e).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "lee(t(c)o)de",
            Solution::min_remove_to_make_valid(String::from("lee(t(c)o)de)"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "ab(c)d",
            Solution::min_remove_to_make_valid(String::from("a)b(c)d"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!("", Solution::min_remove_to_make_valid(String::from("))((")));
    }

    #[test]
    fn example_4() {
        assert_eq!(
            "a(b(c)d)",
            Solution::min_remove_to_make_valid(String::from("(a(b(c)d)"))
        );
    }
}
