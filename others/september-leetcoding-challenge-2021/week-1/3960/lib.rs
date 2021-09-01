pub struct Solution;

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut v = vec![false; nums.len()];
        let mut answer = 0;
        for i in 0..nums.len() {
            let mut i = i;
            let mut len = 0;
            while !v[i] {
                len += 1;
                v[i] = true;
                i = nums[i] as usize;
            }
            answer = answer.max(len);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::array_nesting(vec![0, 1, 2]));
    }
}
