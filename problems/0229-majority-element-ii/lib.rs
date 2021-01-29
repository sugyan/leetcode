pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut candidates: (Option<i32>, Option<i32>) = (None, None);
        let mut counts = (0, 0);
        for num in nums.iter() {
            if Some(*num) == candidates.0 {
                counts.0 += 1;
            } else if Some(*num) == candidates.1 {
                counts.1 += 1;
            } else if counts.0 == 0 {
                candidates.0 = Some(*num);
                counts.0 = 1;
            } else if counts.1 == 0 {
                candidates.1 = Some(*num);
                counts.1 = 1;
            } else {
                counts = (counts.0 - 1, counts.1 - 1);
            }
        }
        let mut answer: Vec<i32> = Vec::new();
        for candidate in [candidates.0, candidates.1].iter() {
            if let Some(c) = candidate {
                if nums.iter().filter(|&n| n == c).count() > nums.len() / 3 {
                    answer.push(*c);
                }
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
        let mut ret = Solution::majority_element(vec![3, 2, 3]);
        ret.sort_unstable();
        assert_eq!(vec![3], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]);
        ret.sort_unstable();
        assert_eq!(vec![1, 2], ret);
    }
}
