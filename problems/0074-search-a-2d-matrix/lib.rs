pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let i = if let Some(i) = (1..matrix.len()).find(|i| matrix[*i][0] > target) {
            i - 1
        } else {
            matrix.len() - 1
        };
        let (mut l, mut r) = (0, matrix[0].len());
        while r - l > 1 {
            let m = l + (r - l) / 2;
            match matrix[i][m].cmp(&target) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => r = m,
                std::cmp::Ordering::Less => l = m,
            }
        }
        return matrix[i][l] == target;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                3
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                13
            )
        );
    }
}
