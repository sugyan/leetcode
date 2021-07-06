use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut hm = HashMap::new();
        for &a in &arr {
            *hm.entry(a).or_default() += 1;
        }
        let mut v = hm.values().collect::<Vec<_>>();
        v.sort_unstable();
        v.iter()
            .rev()
            .scan(0, |state, &x| {
                *state += x;
                if *state * 2 < arr.len() {
                    Some(*state)
                } else {
                    None
                }
            })
            .count() as i32
            + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::min_set_size(vec![1, 9]));
    }

    #[test]
    fn example_4() {
        assert_eq!(1, Solution::min_set_size(vec![1000, 1000, 3, 7]));
    }

    #[test]
    fn example_5() {
        assert_eq!(
            5,
            Solution::min_set_size(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        );
    }
}
