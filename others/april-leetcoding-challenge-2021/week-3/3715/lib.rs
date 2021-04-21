pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut v = vec![std::i32::MAX; triangle.len()];
        v[0] = 0;
        for row in &triangle {
            for (i, &x) in row.iter().enumerate().rev() {
                v[i] = (if i > 0 { v[i].min(v[i - 1]) } else { v[i] }) + x;
            }
        }
        *v.iter().min().unwrap()
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

    #[test]
    fn example_2() {
        assert_eq!(-10, Solution::minimum_total(vec![vec![-10]]));
    }
}
