pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        let mut v = Vec::new();
        Solution::dfs(&mut answer, n, k, 0, &mut v);
        return answer;
    }
    fn dfs(answer: &mut Vec<Vec<i32>>, n: i32, k: i32, j: i32, v: &mut Vec<i32>) {
        if v.len() == k as usize {
            answer.push(v.clone());
            return;
        }
        for i in j..n {
            if n - i < k - v.len() as i32 {
                break;
            }
            v.push(i + 1);
            Solution::dfs(answer, n, k, i + 1, v);
            v.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::combine(4, 2);
        ret.sort();
        assert_eq!(
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ],
            ret
        );
    }
}
