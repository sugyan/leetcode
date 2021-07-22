pub struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut v = nums.clone();
        for i in (0..v.len() - 1).rev() {
            v[i] = v[i].min(v[i + 1]);
        }
        let mut max = 0;
        for i in 0..nums.len() - 1 {
            max = max.max(nums[i]);
            if max <= v[i + 1] {
                return i as i32 + 1;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::partition_disjoint(vec![5, 0, 3, 8, 6]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]));
    }
}
