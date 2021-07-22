pub struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut answer = 1;
        let (mut cmax, mut pmax) = (nums[0], nums[0]);
        for (i, &num) in nums.iter().enumerate().skip(1) {
            if num < pmax {
                answer = i as i32 + 1;
                pmax = cmax;
            } else {
                cmax = cmax.max(num);
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
        assert_eq!(3, Solution::partition_disjoint(vec![5, 0, 3, 8, 6]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]));
    }
}
