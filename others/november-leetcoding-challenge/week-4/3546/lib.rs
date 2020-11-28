use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut btm: BTreeMap<i32, usize> = BTreeMap::new();
        for &num in nums.iter().take(k as usize - 1) {
            *btm.entry(num).or_insert(0) += 1;
        }
        let mut answer = Vec::with_capacity(nums.len() - k as usize + 1);
        for i in k as usize - 1..nums.len() {
            *btm.entry(nums[i]).or_insert(0) += 1;
            if let Some(&last) = btm.keys().last() {
                answer.push(last);
            }
            if let Some(n) = btm.get_mut(&nums[i + 1 - k as usize]) {
                *n -= 1;
                if *n == 0 {
                    btm.remove(&nums[i + 1 - k as usize]);
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![3, 3, 5, 5, 6, 7],
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1], Solution::max_sliding_window(vec![1], 1));
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![1, -1], Solution::max_sliding_window(vec![1, -1], 1));
    }

    #[test]
    fn example_4() {
        assert_eq!(vec![11], Solution::max_sliding_window(vec![9, 11], 2));
    }

    #[test]
    fn example_5() {
        assert_eq!(vec![4], Solution::max_sliding_window(vec![4, -2], 2));
    }
}
