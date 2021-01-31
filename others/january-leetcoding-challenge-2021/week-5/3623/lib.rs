pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if let Some(i) = (1..nums.len()).rev().find(|&i| nums[i - 1] < nums[i]) {
            let j = (i..nums.len())
                .rev()
                .find(|&j| nums[j] > nums[i - 1])
                .unwrap_or(nums.len() - 1);
            nums.swap(i - 1, j);
            nums[i..].reverse();
        } else {
            nums.reverse();
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

    #[test]
    fn example_4() {
        let mut nums = vec![1];
        Solution::next_permutation(&mut nums);
        assert_eq!(vec![1], nums);
    }
}
