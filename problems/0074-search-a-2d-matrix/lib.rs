pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        match matrix.binary_search_by(|row| row[0].cmp(&target)) {
            Ok(_) => true,
            Err(i) => i > 0 && matrix[i - 1].binary_search(&target).is_ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[rustfmt::skip]
    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![
                    vec![ 1,  3,  5,  7],
                    vec![10, 11, 16, 20],
                    vec![23, 30, 34, 60]
                ],
                3
            )
        );
    }

    #[allow(clippy::bool_assert_comparison)]
    #[rustfmt::skip]
    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::search_matrix(
                vec![
                    vec![ 1,  3,  5,  7],
                    vec![10, 11, 16, 20],
                    vec![23, 30, 34, 60]
                ],
                13
            )
        );
    }
}
