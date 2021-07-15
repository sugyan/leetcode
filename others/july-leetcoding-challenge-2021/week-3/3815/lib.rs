use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut answer = 0;
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                if let Err(n) = (j + 1..nums.len())
                    .collect::<Vec<_>>()
                    .binary_search_by(|&k| {
                        if nums[k] < nums[i] + nums[j] {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                {
                    answer += n;
                }
            }
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::triangle_number(vec![2, 2, 3, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::triangle_number(vec![4, 2, 3, 4]));
    }
}
