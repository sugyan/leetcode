use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut answer = std::i32::MIN;
        for i in 0..row {
            let mut rowsum = vec![0; col];
            for r in matrix.iter().skip(i) {
                rowsum.iter_mut().zip(r).for_each(|(sum, val)| *sum += val);
                let mut sum = 0;
                let mut bts = BTreeSet::new();
                bts.insert(0);
                for val in &rowsum {
                    sum += val;
                    if let Some(x) = bts.range(sum - k..).next() {
                        answer = answer.max(sum - x);
                        if answer == k {
                            return k;
                        }
                    };
                    bts.insert(sum);
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3));
    }
}
