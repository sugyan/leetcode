use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut dead = vec![false; 10_000];
        deadends.iter().for_each(|d| {
            if let Ok(i) = d.parse::<usize>() {
                dead[i] = true
            }
        });
        let target = target.parse::<usize>().unwrap();
        let mut min = vec![None; 10_000];
        let mut bh = BinaryHeap::new();
        if !dead[0] {
            bh.push((Reverse(0), 0));
        }
        while let Some((Reverse(d), n)) = bh.pop() {
            if n == target {
                return d;
            }
            for i in 0..4 {
                let j = 10_usize.pow(i + 1);
                for &k in &[
                    n / j * j + (n + 10_usize.pow(i)) % j,
                    n / j * j + (n + j - 10_usize.pow(i)) % j,
                ] {
                    if dead[k] {
                        continue;
                    }
                    if min[k].is_none() {
                        min[k] = Some(d);
                        bh.push((Reverse(d + 1), k));
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            6,
            Solution::open_lock(
                vec!["0201", "0101", "0102", "1212", "2002"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                String::from("0202")
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::open_lock(
                vec!["8888"].into_iter().map(String::from).collect(),
                String::from("0009")
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            -1,
            Solution::open_lock(
                vec!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                String::from("8888")
            )
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            -1,
            Solution::open_lock(
                vec!["0000"].into_iter().map(String::from).collect(),
                String::from("8888")
            )
        );
    }
}
