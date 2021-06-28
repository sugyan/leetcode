pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        s.chars()
            .fold(Vec::new(), |mut stack, c| {
                if stack.last() == Some(&c) {
                    stack.pop();
                } else {
                    stack.push(c);
                }
                stack
            })
            .iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("ca", Solution::remove_duplicates(String::from("abbaca")));
    }

    #[test]
    fn example_2() {
        assert_eq!("ay", Solution::remove_duplicates(String::from("azxxzy")));
    }
}
