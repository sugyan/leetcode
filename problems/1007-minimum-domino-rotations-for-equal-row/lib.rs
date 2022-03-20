pub struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        (1..=6)
            .filter_map(|num| {
                let mut rotations = (0, 0);
                for domino in tops.iter().zip(&bottoms) {
                    match (*domino.0 == num, *domino.1 == num) {
                        (true, true) => {}
                        (true, false) => rotations.0 += 1,
                        (false, true) => rotations.1 += 1,
                        (false, false) => return None,
                    }
                }
                Some(rotations.0.min(rotations.1))
            })
            .min()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            -1,
            Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4])
        );
    }
}
