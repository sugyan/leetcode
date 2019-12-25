pub struct Solution {}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::with_capacity(1 << n);
        answer.push(0);
        for i in 0..n as usize {
            let m = 1 << i;
            for j in (0..answer.len()).rev() {
                answer.push(m | answer[j]);
            }
        }
        return answer;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let ret = Solution::gray_code(2);
        assert!(ret == vec![0, 1, 3, 2] || ret == vec![0, 2, 3, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![0], Solution::gray_code(0));
    }
}
