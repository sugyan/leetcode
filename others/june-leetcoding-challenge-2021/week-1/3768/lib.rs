use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DIV: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut v = efficiency
            .iter()
            .zip(speed.iter())
            .map(|(&e, &s)| (Reverse(e as i64), s as i64))
            .collect::<Vec<_>>();
        v.sort_unstable();
        let mut bh = BinaryHeap::with_capacity(k as usize);
        (v.iter()
            .scan(0, |sum, (Reverse(e), s)| {
                *sum += s;
                bh.push(Reverse(s));
                if bh.len() > k as usize {
                    if let Some(Reverse(min)) = bh.pop() {
                        *sum -= min;
                    }
                }
                Some(*sum * e)
            })
            .max()
            .unwrap()
            % DIV) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            60,
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            68,
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            72,
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4)
        );
    }
}
