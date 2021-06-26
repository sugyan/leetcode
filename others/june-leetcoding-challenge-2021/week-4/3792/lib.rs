use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::with_capacity(nums.len());
        let mut answer = vec![0; nums.len()];
        for (i, num) in nums.iter().enumerate().rev() {
            let j = v
                .binary_search_by(|&probe| {
                    if probe >= num {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                })
                .unwrap_err();
            answer[i] = j as i32;
            v.insert(j, num);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![2, 1, 1, 0], Solution::count_smaller(vec![5, 2, 6, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![0], Solution::count_smaller(vec![-1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![0, 0], Solution::count_smaller(vec![-1, -1]));
    }
}
