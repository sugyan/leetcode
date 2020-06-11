pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut i = 0;
        while i <= r {
            match nums[i] {
                0 => {
                    nums.swap(l, i);
                    l += 1;
                    i += 1;
                }
                2 => {
                    nums.swap(r, i);
                    if r == 0 {
                        break;
                    }
                    r -= 1;
                }
                _ => i += 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);
    }
}
