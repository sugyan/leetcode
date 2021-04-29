pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lower = {
            let (mut lo, mut hi) = (0, nums.len());
            while lo < hi {
                let m = (lo + hi) / 2;
                if nums[m] < target {
                    lo = m + 1
                } else {
                    hi = m
                }
            }
            lo as i32
        };
        let upper = {
            let (mut lo, mut hi) = (0, nums.len());
            while lo < hi {
                let m = (lo + hi) / 2;
                if nums[m] <= target {
                    lo = m + 1
                } else {
                    hi = m
                }
            }
            lo as i32 - 1
        };
        if lower > upper {
            [-1, -1].to_vec()
        } else {
            [lower, upper].to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 0));
    }
}
