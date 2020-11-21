pub struct Solution {}

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let digits: Vec<char> = digits.iter().map(|s| s.chars().next().unwrap()).collect();
        let n = n.to_string();
        let mut dp = vec![0; n.len() + 1];
        dp[n.len()] = 1;
        for (i, c) in n.chars().rev().enumerate() {
            for &d in digits.iter() {
                match d.cmp(&c) {
                    std::cmp::Ordering::Less => dp[n.len() - i - 1] += digits.len().pow(i as u32),
                    std::cmp::Ordering::Equal => dp[n.len() - i - 1] += dp[n.len() - i],
                    std::cmp::Ordering::Greater => {}
                }
            }
        }
        (dp[0]
            + (1..n.len())
                .map(|i| digits.len().pow(i as u32))
                .sum::<usize>()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            20,
            Solution::at_most_n_given_digit_set(
                vec![
                    String::from("1"),
                    String::from("3"),
                    String::from("5"),
                    String::from("7")
                ],
                100
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            29523,
            Solution::at_most_n_given_digit_set(
                vec![String::from("1"), String::from("4"), String::from("9")],
                1000000000
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            1,
            Solution::at_most_n_given_digit_set(vec![String::from("7")], 8)
        );
    }
}
