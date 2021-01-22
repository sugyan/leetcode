pub struct Solution {}

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut v = (1..=n as usize).collect::<Vec<_>>();
        Self::backtrack(&mut v, n as usize)
    }
    fn backtrack(v: &mut Vec<usize>, i: usize) -> i32 {
        if i < 2 {
            1
        } else {
            let mut ret = 0;
            let candidates = (0..i)
                .filter(|&j| v[j] % i == 0 || i % v[j] == 0)
                .collect::<Vec<usize>>();
            for &j in candidates.iter() {
                v.swap(j, i - 1);
                ret += Solution::backtrack(v, i - 1);
                v.swap(j, i - 1);
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
