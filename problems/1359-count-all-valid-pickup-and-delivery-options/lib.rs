pub struct Solution;

const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut answer = 1;
        for i in 0..n as u64 {
            answer = answer * (i * 2 + 1) % MOD;
            answer = answer * (i + 1) % MOD;
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::count_orders(1));
    }

    #[test]
    fn example_2() {
        assert_eq!(6, Solution::count_orders(2));
    }

    #[test]
    fn example_3() {
        assert_eq!(90, Solution::count_orders(3));
    }
}
