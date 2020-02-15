pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        Solution::quick_select(&mut nums, 0, len - 1, k as usize)
    }
    fn quick_select(nums: &mut [i32], l: usize, r: usize, k: usize) -> i32 {
        let pivot = {
            let mut v: Vec<usize> = vec![l, (l + r) / 2, r];
            v.sort_by_key(|i| nums[*i]);
            v[1]
        };
        nums.swap(pivot, r);
        let mut n = l;
        for i in l..r {
            if nums[i] < nums[r] {
                nums.swap(i, n);
                n += 1;
            }
        }
        let count = r - n + 1;
        match count.cmp(&k) {
            std::cmp::Ordering::Equal => nums[r],
            std::cmp::Ordering::Greater => Solution::quick_select(nums, n, r - 1, k),
            std::cmp::Ordering::Less => Solution::quick_select(nums, l, n - 1, k - count),
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
