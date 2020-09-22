pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut candidates = (0, 0);
        let mut counts = (0, 0);
        for &num in nums.iter() {
            if num == candidates.0 {
                counts.0 += 1;
            } else if num == candidates.1 {
                counts.1 += 1;
            } else if counts.0 == 0 {
                candidates.0 = num;
                counts.0 = 1;
            } else if counts.1 == 0 {
                candidates.1 = num;
                counts.1 = 1;
            } else {
                counts = (counts.0 - 1, counts.1 - 1);
            }
        }
        counts = (0, 0);
        for &num in nums.iter() {
            if num == candidates.0 {
                counts.0 += 1;
            } else if num == candidates.1 {
                counts.1 += 1;
            }
        }
        let mut answer = Vec::new();
        if counts.0 > nums.len() / 3 {
            answer.push(candidates.0);
        }
        if counts.1 > nums.len() / 3 {
            answer.push(candidates.1);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![3], Solution::majority_element(vec![3, 2, 3]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1, 2],
            Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2])
        );
    }
}
