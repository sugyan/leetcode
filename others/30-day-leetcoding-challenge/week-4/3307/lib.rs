use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        hm.insert(0, 1);
        let mut answer = 0;
        let mut sum = 0;
        for num in nums.iter() {
            sum += num;
            if let Some(v) = hm.get(&(sum - k)) {
                answer += v;
            }
            *hm.entry(sum).or_insert(0) += 1;
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
    }
}
