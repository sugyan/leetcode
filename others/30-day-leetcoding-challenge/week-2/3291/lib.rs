pub struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut vs: Vec<char> = Vec::new();
        let mut vt: Vec<char> = Vec::new();
        for c in s.chars() {
            if c == '#' {
                vs.pop();
            } else {
                vs.push(c)
            }
        }
        for c in t.chars() {
            if c == '#' {
                vt.pop();
            } else {
                vt.push(c)
            }
        }
        vs == vt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::backspace_compare("ab##".to_string(), "c#d#".to_string())
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::backspace_compare("a##c".to_string(), "#a#c".to_string())
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            false,
            Solution::backspace_compare("a#c".to_string(), "b".to_string())
        );
    }
}
