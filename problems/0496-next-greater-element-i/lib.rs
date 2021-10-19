use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hm = HashMap::new();
        let mut stack = Vec::new();
        for &num in &nums2 {
            while let Some(&last) = stack.last() {
                if num > last {
                    stack.pop();
                    hm.insert(last, num);
                } else {
                    break;
                }
            }
            stack.push(num);
        }
        let mut answer = Vec::with_capacity(nums1.len());
        for num in &nums1 {
            answer.push(*hm.get(num).unwrap_or(&-1));
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
            vec![-1, 3, -1],
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![3, -1],
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4])
        );
    }
}
