pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = Vec::new();
        let mut flags = vec![true; s.len()];
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => stack.push(i),
                ')' if stack.pop().is_none() => flags[i] = false,
                _ => {}
            }
        }
        while let Some(i) = stack.pop() {
            flags[i] = false;
        }
        s.chars()
            .enumerate()
            .filter(|&(i, _)| flags[i])
            .map(|(_, c)| c)
            .collect()
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
}
