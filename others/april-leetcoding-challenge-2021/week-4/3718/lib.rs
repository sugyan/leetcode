pub struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .fold(vec![1], |mut v, w| {
                if w[0] == w[1] {
                    *v.last_mut().unwrap() += 1;
                } else {
                    v.push(1);
                }
                v
            })
            .windows(2)
            .map(|w| w[0].min(w[1]))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            6,
            Solution::count_binary_substrings(String::from("00110011"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::count_binary_substrings(String::from("10101")));
    }
}
