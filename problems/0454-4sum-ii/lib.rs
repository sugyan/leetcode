use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let (mut hm12, mut hm34) = (HashMap::new(), HashMap::new());
        for num1 in &nums1 {
            for num2 in &nums2 {
                *hm12.entry(num1 + num2).or_insert(0) += 1;
            }
        }
        for num3 in &nums3 {
            for num4 in &nums4 {
                *hm34.entry(num3 + num4).or_insert(0) += 1;
            }
        }
        hm12.iter()
            .map(|(k12, v12)| v12 * hm34.get(&(-k12)).unwrap_or(&0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0])
        );
    }
}
