use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        hm.insert(0, 0);
        let mut answer = 0;
        let mut n = 0;
        for (i, num) in (0..).zip(nums.iter()) {
            n += if *num == 1 { 1 } else { -1 };
            if let Some(j) = hm.get(&n) {
                answer = std::cmp::max(answer, i - j + 1);
            } else {
                hm.insert(n, i + 1);
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
        assert_eq!(2, Solution::find_max_length(vec![0, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::find_max_length(vec![0, 1, 0]));
    }
}
