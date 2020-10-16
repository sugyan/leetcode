pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        match matrix.binary_search_by(|row: &Vec<i32>| row[0].cmp(&target)) {
            Ok(_) => true,
            Err(i) => (0..i).any(|i: usize| matrix[i].binary_search(&target).is_ok()),
        }
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

    #[test]
    fn example_3() {
        assert_eq!(false, Solution::search_matrix(vec![], 0));
    }
}
