pub struct Solution;

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums.iter().filter(|&n| *n > 0).collect::<Vec<_>>();
        if nums.len() < 3 {
            return 0;
        }
        nums.sort_unstable();
        let mut answer = 0;
        for i in 0..nums.len() - 2 {
            let mut k = i + 2;
            for j in i + 1..nums.len() - 1 {
                while k < nums.len() && nums[i] + nums[j] > *nums[k] {
                    k += 1;
                }
                answer += k - j - 1;
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
