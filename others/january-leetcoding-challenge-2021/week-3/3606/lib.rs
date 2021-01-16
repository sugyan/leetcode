pub struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let lr = (0, nums.len() - 1);
        Self::quick_select(&mut nums, lr, k as usize)
    }
    fn quick_select(nums: &mut [i32], lr: (usize, usize), k: usize) -> i32 {
        let pivot = (lr.0 + lr.1) / 2;
        nums.swap(pivot, lr.1);
        let mut n = lr.0;
        for i in lr.0..lr.1 {
            if nums[i] < nums[lr.1] {
                nums.swap(i, n);
                n += 1;
            }
        }
        let count = lr.1 - n + 1;
        match count.cmp(&k) {
            std::cmp::Ordering::Less => Self::quick_select(nums, (lr.0, n - 1), k - count),
            std::cmp::Ordering::Equal => nums[lr.1],
            std::cmp::Ordering::Greater => Self::quick_select(nums, (n, lr.1 - 1), k),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
        );
    }
}
