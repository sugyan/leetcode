pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        let mut i = 0;
        while i <= hi {
            match nums[i] {
                0 => {
                    nums.swap(lo, i);
                    lo += 1;
                    i += 1;
                }
                1 => i += 1,
                2 => {
                    nums.swap(hi, i);
                    if hi == 0 {
                        break;
                    }
                    hi -= 1;
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 1, 2], nums);
    }

    #[test]
    fn example_3() {
        let mut nums = vec![0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0], nums);
    }

    #[test]
    fn example_4() {
        let mut nums = vec![1];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![1], nums);
    }
}
