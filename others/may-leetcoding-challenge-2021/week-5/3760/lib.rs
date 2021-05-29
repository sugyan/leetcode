pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut v = Vec::new();
        Self::backtrack(n, &mut v)
    }
    fn backtrack(n: i32, v: &mut Vec<i32>) -> i32 {
        if v.len() == n as usize {
            return 1;
        }
        let mut ret = 0;
        for i in 0..n {
            if v.iter()
                .enumerate()
                .any(|(j, &q)| q == i || (v.len() - j) as i32 == (q - i).abs())
            {
                continue;
            }
            v.push(i);
            ret += Self::backtrack(n, v);
            v.pop();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::total_n_queens(4));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::total_n_queens(1));
    }
}
