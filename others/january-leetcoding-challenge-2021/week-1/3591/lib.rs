use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut hs = (1..=n as usize).collect::<HashSet<_>>();
        Self::backtrack(&mut hs)
    }
    fn backtrack(hs: &mut HashSet<usize>) -> i32 {
        let len = hs.len();
        if len <= 1 {
            return 1;
        } else {
            let mut ret = 0;
            for c in hs
                .iter()
                .filter(|&m| m % len == 0 || len % m == 0)
                .map(|&m| m)
                .collect::<Vec<usize>>()
            {
                hs.remove(&c);
                ret += Solution::backtrack(hs);
                hs.insert(c);
            }
            ret
        }
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
