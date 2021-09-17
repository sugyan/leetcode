pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut v = vec![0; 1001];
        for &num in &nums1 {
            v[num as usize] += 1;
        }
        nums2
            .into_iter()
            .filter(|&num| {
                v[num as usize] -= 1;
                v[num as usize] >= 0
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]);
        ret.sort();
        assert_eq!(vec![2, 2], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        ret.sort();
        assert_eq!(vec![4, 9], ret);
    }
}
