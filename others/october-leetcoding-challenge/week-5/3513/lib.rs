use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = Vec::new();
        let mut hm: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();
        for &num in nums.iter() {
            let i = match dp.binary_search(&num) {
                Ok(i) | Err(i) => i,
            };
            if i == dp.len() {
                dp.push(num);
            } else {
                dp[i] = num;
            }
            let total = if let Some(v) = hm.get(&i) {
                v.iter()
                    .filter(|&(_, last)| *last < num)
                    .map(|&(count, _)| count)
                    .sum()
            } else {
                1
            };
            hm.entry(i + 1).or_insert_with(Vec::new).push((total, num));
        }
        if let Some(v) = hm.get(&dp.len()) {
            v.iter().map(|&(count, _)| count).sum::<usize>() as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]));
    }

    #[test]
    fn example_2() {
        assert_eq!(5, Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]));
    }
}
