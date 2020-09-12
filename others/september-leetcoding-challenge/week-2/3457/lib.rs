pub struct Solution {}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut v: Vec<i32> = Vec::new();
        Solution::helper(k, n, &mut v, &mut answer);
        answer
    }
    fn helper(k: i32, n: i32, v: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
        if k == 0 && n == 0 {
            answer.push(v.clone());
            return;
        }
        let last = v.last().unwrap_or(&0);
        for i in last + 1..=std::cmp::min(n, 9) {
            v.push(i);
            Solution::helper(k - 1, n - i, v, answer);
            v.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![vec![1, 2, 4]], Solution::combination_sum3(3, 7));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]],
            Solution::combination_sum3(3, 9)
        );
    }
}
