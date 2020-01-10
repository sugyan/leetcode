pub struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if let Some(v) = triangle.last() {
            let mut v = v.clone();
            for i in (0..triangle.len() - 1).rev() {
                for j in 0..=i {
                    v[j] = triangle[i][j] + std::cmp::min(v[j], v[j + 1]);
                }
            }
            v[0]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            11,
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        );
    }
}
