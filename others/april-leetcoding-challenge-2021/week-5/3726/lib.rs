use std::collections::HashSet;
use std::iter::successors;

pub struct Solution;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        successors(Some(1), |n| Some(n * x).filter(|&xi| x > 1 && xi < bound))
            .flat_map(|xi| {
                successors(Some(1), move |n| {
                    Some(n * y).filter(|&yj| y > 1 && yj <= bound - xi)
                })
                .filter_map(move |yj| Some(xi + yj).filter(|&sum| sum <= bound))
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::powerful_integers(2, 3, 10);
        ret.sort_unstable();
        assert_eq!(vec![2, 3, 4, 5, 7, 9, 10], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::powerful_integers(3, 5, 15);
        ret.sort_unstable();
        assert_eq!(vec![2, 4, 6, 8, 10, 14], ret);
    }
}
