use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm = HashMap::from([(0, 1)]);
        let (mut sum, mut answer) = (0, 0);
        for num in &nums {
            sum += num;
            answer += hm.get(&(sum - k)).unwrap_or(&0);
            *hm.entry(sum).or_insert(0) += 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 2, 3], 3));
    }
}
