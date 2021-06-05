use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DIV: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut e = (0..n as usize).collect::<Vec<_>>();
        e.sort_by_key(|&i| Reverse(efficiency[i]));
        let mut bh = BinaryHeap::with_capacity(k as usize);
        let mut sum = 0;
        let mut answer = 0;
        for i in 0..n as usize {
            sum += speed[e[i]] as i64;
            bh.push(Reverse(speed[e[i]] as i64));
            if bh.len() > k as usize {
                if let Some(Reverse(min)) = bh.pop() {
                    sum -= min;
                }
            }
            answer = answer.max(sum * efficiency[e[i]] as i64);
        }
        (answer % DIV) as i32
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
