use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes
            .iter()
            .map(|envelope| (envelope[0], Reverse(envelope[1])))
            .collect::<Vec<_>>();
        envelopes.sort_unstable();
        let mut dp = Vec::new();
        for &(_, Reverse(h)) in &envelopes {
            let i = match dp.binary_search(&h) {
                Ok(i) | Err(i) => i,
            };
            if i == dp.len() {
                dp.push(h);
            } else {
                dp[i] = h;
            }
        }
        dp.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1]])
        );
    }
}
