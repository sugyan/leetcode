pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        if a.len() < 3 {
            return 0;
        }
        let mut answer = 0;
        let mut i = 0;
        while i < a.len() - 2 {
            if a[i] - a[i + 1] == a[i + 1] - a[i + 2] {
                let mut n = 1;
                while i + 3 < a.len() && a[i + 1] - a[i + 2] == a[i + 2] - a[i + 3] {
                    n += 1;
                    i += 1;
                }
                answer += n * (n + 1) / 2;
            }
            i += 1;
        }
        answer
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
