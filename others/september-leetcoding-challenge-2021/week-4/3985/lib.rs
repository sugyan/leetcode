pub struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut v = palindrome.chars().collect::<Vec<_>>();
        if v.len() == 1 {
            return String::new();
        }
        if let Some(i) =
            (0..v.len()).position(|i| v[i] != 'a' && (v.len() % 2 == 0 || i != v.len() / 2))
        {
            v[i] = 'a';
        } else {
            *v.last_mut().unwrap() = 'b'
        }
        v.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("aaccba", Solution::break_palindrome(String::from("abccba")));
    }

    #[test]
    fn example_2() {
        assert_eq!("", Solution::break_palindrome(String::from("a")));
    }

    #[test]
    fn example_3() {
        assert_eq!("ab", Solution::break_palindrome(String::from("aa")));
    }

    #[test]
    fn example_4() {
        assert_eq!("abb", Solution::break_palindrome(String::from("aba")));
    }
}
