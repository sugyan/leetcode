use std::cmp::Ordering;
use std::collections::HashMap;

const MOD: usize = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let counts = arr
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<i32, usize>, &x| {
                *acc.entry(x).or_default() += 1;
                acc
            });
        let mut answer = 0;
        for (&k1, &v1) in counts.iter() {
            if k1 * 3 == target && v1 >= 3 {
                answer += v1 * (v1 - 1) * (v1 - 2) / 6;
            }
            if target - k1 * 2 > k1 && v1 >= 2 {
                if let Some(&v2) = counts.get(&(target - k1 * 2)) {
                    answer += v1 * (v1 - 1) / 2 * v2;
                }
            }
            for (&k2, &v2) in counts.iter().filter(|(&k, _)| k > k1) {
                match (target - k1 - k2).cmp(&k2) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        if v2 >= 2 {
                            answer += v1 * v2 * (v2 - 1) / 2;
                        }
                    }
                    Ordering::Greater => {
                        if let Some(&v3) = counts.get(&(target - k1 - k2)) {
                            answer += v1 * v2 * v3;
                        }
                    }
                }
            }
        }
        (answer % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            20,
            Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(12, Solution::three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5));
    }
}
