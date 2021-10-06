pub struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut answer = Vec::new();
        for i in 0..nums.len() {
            let val = nums[i].abs();
            let idx = val as usize - 1;
            nums[idx] *= -1;
            if nums[idx] > 0 {
                answer.push(val);
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
        assert_eq!(
            vec![2, 3],
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1], Solution::find_duplicates(vec![1, 1, 2]));
    }

    #[test]
    fn example_3() {
        assert_eq!(Vec::<i32>::new(), Solution::find_duplicates(vec![1]));
    }
}
