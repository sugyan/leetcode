use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let mut hs: HashSet<i32> = HashSet::new();
        hs.insert(0);
        for &num in nums.iter() {
            for n in hs
                .iter()
                .map(|&n| n + num)
                .filter(|&n| n <= sum / 2)
                .collect::<Vec<i32>>()
            {
                if n == sum / 2 {
                    return true;
                }
                hs.insert(n);
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
