pub struct Solution {}

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<i32> = nums;
        let mut answer: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            let val = nums[i].abs();
            let idx = val as usize - 1;
            nums[idx] *= -1;
            if nums[idx] > 0 {
                answer.push(val);
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
        let mut ret = Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]);
        ret.sort();
        assert_eq!(vec![2, 3], ret);
    }
}
