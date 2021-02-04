use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut hm = HashMap::new();
        nums.iter()
            .for_each(|&num| *hm.entry(num).or_insert(0) += 1);
        let mut answer = 0;
        for (&num, &count) in &hm {
            if let Some(c) = hm.get(&(num + 1)) {
                answer = std::cmp::max(answer, count + c);
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
        assert_eq!(5, Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::find_lhs(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::find_lhs(vec![1, 1, 1, 1]));
    }
}
