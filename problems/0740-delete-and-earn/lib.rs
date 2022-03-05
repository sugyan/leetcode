use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut btm = BTreeMap::new();
        for &num in &nums {
            *btm.entry(num).or_insert(0) += num;
        }
        let keys = btm.keys().collect::<Vec<_>>();
        let mut prev = (0, btm[keys[0]]);
        for i in 1..keys.len() {
            prev = (
                prev.1,
                if *keys[i] == *keys[i - 1] + 1 {
                    prev.1.max(prev.0 + btm[keys[i]])
                } else {
                    prev.1 + btm[keys[i]]
                },
            );
        }
        prev.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::delete_and_earn(vec![3, 4, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(9, Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]));
    }
}
