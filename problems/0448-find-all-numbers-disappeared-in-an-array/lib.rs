pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            let mut j = nums[i] as usize - 1;
            while j != nums[j] as usize - 1 {
                nums.swap(i, j);
                j = nums[i] as usize - 1;
            }
        }
        (1..=nums.len() as i32)
            .filter(|&i| nums[i as usize - 1] != i)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![5, 6],
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![2], Solution::find_disappeared_numbers(vec![1, 1]));
    }
}
