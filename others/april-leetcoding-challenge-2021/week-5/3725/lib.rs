use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lower = nums.binary_search_by(|p| match p.cmp(&target) {
            Ordering::Equal => Ordering::Greater,
            o => o,
        });
        let upper = nums.binary_search_by(|p| match p.cmp(&target) {
            Ordering::Equal => Ordering::Less,
            o => o,
        });
        if let (Err(l), Err(u)) = (lower, upper) {
            if l < u {
                return [l as i32, u as i32 - 1].to_vec();
            }
        }
        [-1, -1].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 0));
    }
}
