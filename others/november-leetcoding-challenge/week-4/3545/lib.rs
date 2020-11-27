pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum % 2 != 0 {
            return false;
        }
        let mut v: Vec<bool> = vec![false; sum / 2 + 1];
        v[0] = true;
        for &num in nums.iter() {
            for i in (num as usize..=sum / 2).rev() {
                v[i] |= v[i - num as usize];
            }
            if v[sum / 2] {
                return true;
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
        assert_eq!(true, Solution::can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::can_partition(vec![1, 2, 3, 5]));
    }
}
