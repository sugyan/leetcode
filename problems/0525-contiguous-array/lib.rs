use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut hm = vec![(0, -1)].into_iter().collect::<HashMap<_, _>>();
        let (mut n, mut answer) = (0, 0);
        for (i, &num) in (0..).zip(&nums) {
            n += if num == 1 { 1 } else { -1 };
            if let Some(j) = hm.get(&n) {
                answer = answer.max(i - j);
            } else {
                hm.insert(n, i);
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
        assert_eq!(2, Solution::find_max_length(vec![0, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::find_max_length(vec![0, 1, 0]));
    }
}
