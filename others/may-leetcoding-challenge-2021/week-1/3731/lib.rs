pub struct Solution;

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        nums.windows(2).filter(|w| w[0] > w[1]).count() < 2
            && nums
                .windows(4)
                .all(|w| w[1] <= w[2] || w[0] <= w[2] || w[1] <= w[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::check_possibility(vec![4, 2, 3]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::check_possibility(vec![4, 2, 1]));
    }
}
