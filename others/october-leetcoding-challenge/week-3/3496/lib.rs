pub struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        for i in 0..nums.len() / 2 {
            nums.swap(i, len - 1 - i);
        }
        let k = k as usize % len;
        for i in 0..k / 2 {
            nums.swap(i, k - 1 - i);
        }
        for i in k..(len + k) / 2 {
            nums.swap(i, len - 1 - i + k);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], nums);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(vec![3, 99, -1, -100], nums);
    }
}
