pub struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut v: Vec<i32> = vec![0; n as usize + 1];
        v[0] = 1;
        for i in 1..=n as usize {
            v[i] = (0..i).map(|j| v[j] * v[i - j - 1]).sum();
        }
        v[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::num_trees(3));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::num_trees(1));
    }
}
