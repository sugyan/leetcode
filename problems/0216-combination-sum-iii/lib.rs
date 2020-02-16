pub struct Solution {}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut v: Vec<i32> = Vec::new();
        Solution::helper(&mut answer, &mut v, k as usize, n);
        answer
    }
    fn helper(answer: &mut Vec<Vec<i32>>, v: &mut Vec<i32>, k: usize, n: i32) {
        if v.len() == k && n == 0 {
            answer.push(v.clone());
            return;
        }
        for i in if let Some(last) = v.last() { *last } else { 0 } + 1..=9 {
            if i <= n {
                v.push(i);
                Solution::helper(answer, v, k, n - i);
                v.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::combination_sum3(3, 7);
        ret.sort();
        assert_eq!(vec![vec![1, 2, 4]], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::combination_sum3(3, 9);
        ret.sort();
        assert_eq!(vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]], ret);
    }
}
