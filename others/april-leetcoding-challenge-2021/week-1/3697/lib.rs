pub struct Solution;

impl Solution {
    pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
        a.iter()
            .enumerate()
            .all(|(i, &n)| (n - i as i32).abs() <= 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_ideal_permutation(vec![1, 0, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_ideal_permutation(vec![1, 2, 0]));
    }
}
