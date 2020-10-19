pub struct Solution {}

impl Solution {
    pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
        'nums: for num in 1..=6 {
            let mut rotations = (0, 0);
            for domino in a.iter().zip(b.iter()) {
                match (*domino.0 == num, *domino.1 == num) {
                    (true, true) => {}
                    (true, false) => rotations.0 += 1,
                    (false, true) => rotations.1 += 1,
                    (false, false) => continue 'nums,
                }
            }
            return std::cmp::min(rotations.0, rotations.1);
        }
        -1
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
