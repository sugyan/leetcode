pub struct Solution;

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; mat[0].len()]; mat.len()];
        let (r, c) = (mat.len(), mat[0].len());
        for n in 1..r + c {
            let mut values = Vec::new();
            let mut positions = Vec::new();
            for (i, row) in mat.iter().enumerate() {
                for (j, col) in row.iter().enumerate() {
                    if i + c == n + j {
                        values.push(*col);
                        positions.push((i, j));
                    }
                }
            }
            values.sort_unstable();
            for (k, &(i, j)) in positions.iter().enumerate() {
                ret[i][j] = values[k];
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        #[rustfmt::skip]
        assert_eq!(
            vec![
                vec![1, 1, 1, 1],
                vec![1, 2, 2, 2],
                vec![1, 2, 3, 3]
            ],
            Solution::diagonal_sort(
                vec![
                    vec![3, 3, 1, 1],
                    vec![2, 2, 1, 2],
                    vec![1, 1, 1, 2]
                ]
            )
        );
    }
}
