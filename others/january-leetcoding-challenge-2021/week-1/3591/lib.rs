use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut hm = HashMap::new();
        let mut v = vec![false; n as usize];
        Solution::dfs(&mut v, &mut hm, 1, n as usize)
    }
    fn dfs(v: &mut Vec<bool>, memo: &mut HashMap<Vec<bool>, i32>, i: usize, n: usize) -> i32 {
        if i > n {
            return 1;
        }
        if let Some(&ret) = memo.get(v) {
            return ret;
        }
        let mut ret = 0;
        for &j in (0..n)
            .filter(|&e| !v[e] && ((e + 1) % i == 0 || i % (e + 1) == 0))
            .collect::<Vec<usize>>()
            .iter()
        {
            v[j] = true;
            ret += Solution::dfs(v, memo, i + 1, n);
            v[j] = false;
        }
        memo.insert(v.clone(), ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::count_arrangement(2));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::count_arrangement(1));
    }
}
