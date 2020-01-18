pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut answer = 0;
        for i in 0..32 {
            if nums.iter().filter(|&num| *num & (1 << i) != 0).count() % 3 != 0 {
                answer += 1 << i;
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::single_number(vec![2, 2, 3, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(99, Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
    }
}
