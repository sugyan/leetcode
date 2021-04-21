pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        (0..triangle.len() - 1)
            .rev()
            .fold(triangle[triangle.len() - 1].clone(), |mut acc, i| {
                (0..=i).for_each(|j| acc[j] = triangle[i][j] + acc[j].min(acc[j + 1]));
                acc
            })[0]
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
