pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack = Vec::with_capacity(num.len());
        let mut k = k as usize;
        for c in num.chars() {
            while k > 0 && stack.last().map_or(false, |&last| last > c) {
                stack.pop();
                k -= 1;
            }
            if !stack.is_empty() || c != '0' {
                stack.push(c);
            }
        }
        if stack.len() <= k {
            String::from("0")
        } else {
            stack[..stack.len() - k].iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("1219", Solution::remove_kdigits(String::from("1432219"), 3));
    }

    #[test]
    fn example_2() {
        assert_eq!("200", Solution::remove_kdigits(String::from("10200"), 1));
    }

    #[test]
    fn example_3() {
        assert_eq!("0", Solution::remove_kdigits(String::from("10"), 2));
    }
}
