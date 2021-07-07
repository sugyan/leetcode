pub struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let (mut lo, mut hi) = (matrix[0][0], matrix[n - 1][n - 1]);
        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut count = 0;
            let (mut i, mut j) = (n - 1, 0);
            while j < n {
                if matrix[i][j] > mid {
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                } else {
                    count += i as i32 + 1;
                    j += 1;
                }
            }
            if count < k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            13,
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(-5, Solution::kth_smallest(vec![vec![-5]], 1));
    }
}
