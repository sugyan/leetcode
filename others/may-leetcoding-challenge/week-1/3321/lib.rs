pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut answer, mut count) = (0, 0);
        for num in nums.iter() {
            if count == 0 {
                answer = *num;
            }
            count += if *num == answer { 1 } else { -1 };
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}
