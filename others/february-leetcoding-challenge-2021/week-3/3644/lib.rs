pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut n = 0;
        for i in 2..a.len() {
            if a[i] - a[i - 1] == a[i - 1] - a[i - 2] {
                n += 1;
            } else {
                answer += n * (n + 1) / 2;
                n = 0;
            }
        }
        answer + n * (n + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]));
    }
}
