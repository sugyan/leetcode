pub struct Solution {}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut hs = std::collections::HashSet::new();
        for n in nums.iter() {
            if hs.contains(n) {
                return *n;
            }
            hs.insert(*n);
        }
        0
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
