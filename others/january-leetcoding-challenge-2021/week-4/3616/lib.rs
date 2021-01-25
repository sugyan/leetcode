pub struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        (0..nums.len())
            .filter(|&i| nums[i] == 1)
            .try_fold(None, |prev, i| {
                if let Some(p) = prev {
                    if i - p <= k as usize {
                        return None;
                    }
                }
                Some(Some(i))
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::k_length_apart(vec![1, 1, 1, 1, 1], 0));
    }

    #[test]
    fn example_4() {
        assert_eq!(true, Solution::k_length_apart(vec![0, 1, 0, 1], 1));
    }
}
