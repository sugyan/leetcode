pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        if i > 0 {
            let mut j = r;
            while nums[j] <= nums[i - 1] {
                j -= 1;
            }
            nums.swap(i - 1, j);
            l = i;
        }
        while l < r {
            nums.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(vec![1, 3, 2], nums);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(vec![1, 2, 3], nums);
    }

    #[test]
    fn example_3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(vec![1, 5, 1], nums);
    }
}
