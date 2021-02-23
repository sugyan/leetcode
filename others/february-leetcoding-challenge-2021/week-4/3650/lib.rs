pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut i, mut j) = (0, matrix[0].len() - 1);
        while i < matrix.len() {
            match matrix[i][j].cmp(&target) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater if j > 0 => j -= 1,
                std::cmp::Ordering::Greater => break,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![
                    vec![ 1,  4,  7, 11, 15],
                    vec![ 2,  5,  8, 12, 19],
                    vec![ 3,  6,  9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                5
            )
        );
    }

    #[rustfmt::skip]
    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::search_matrix(
                vec![
                    vec![ 1,  4,  7, 11, 15],
                    vec![ 2,  5,  8, 12, 19],
                    vec![ 3,  6,  9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                20
            )
        );
    }
}
