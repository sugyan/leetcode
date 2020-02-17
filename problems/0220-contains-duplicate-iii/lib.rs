pub struct Solution {}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        for i in 0..nums.len() {
            for j in 0..k as usize {
                if i + 1 + j >= nums.len() {
                    continue;
                }
                if (nums[i] as i64 - nums[i + 1 + j] as i64).abs() <= t as i64 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3)
        );
    }

    #[test]
    fn wa() {
        assert_eq!(
            true,
            Solution::contains_nearby_almost_duplicate(vec![7, 1, 3], 2, 3)
        );
        assert_eq!(
            false,
            Solution::contains_nearby_almost_duplicate(vec![], 0, 0)
        );
        assert_eq!(
            false,
            Solution::contains_nearby_almost_duplicate(vec![-1, 2_147_483_647], 1, 2_147_483_647)
        )
    }
}
