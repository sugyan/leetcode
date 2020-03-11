pub struct Solution {}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut p = (0, 0);
        loop {
            p.0 = nums[p.0 as usize];
            p.1 = nums[p.1 as usize];
            p.1 = nums[p.1 as usize];
            if p.0 == p.1 {
                p.0 = 0;
                while p.0 != p.1 {
                    p.0 = nums[p.0 as usize];
                    p.1 = nums[p.1 as usize];
                }
                return p.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::find_duplicate(vec![1, 3, 4, 2, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::find_duplicate(vec![3, 1, 3, 4, 2]));
    }
}
