pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut answer = Vec::with_capacity(1 << n);
        answer.push(0);
        for i in 0..n {
            for j in (0..answer.len()).rev() {
                answer.push(answer[j] | 1 << i);
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
        assert_eq!(vec![0, 1, 3, 2], Solution::gray_code(2));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![0, 1], Solution::gray_code(1));
    }
}
