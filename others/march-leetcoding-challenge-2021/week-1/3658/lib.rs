pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![0, 0];
        let mut v = vec![false; nums.len()];
        for &num in &nums {
            if v[num as usize - 1] {
                answer[0] = num;
            }
            v[num as usize - 1] = true;
        }
        for (i, &b) in v.iter().enumerate() {
            if !b {
                answer[1] = i as i32 + 1;
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
        assert_eq!(vec![2, 3], Solution::find_error_nums(vec![1, 2, 2, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1, 2], Solution::find_error_nums(vec![1, 1]));
    }
}
