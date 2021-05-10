pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let mut sieve = vec![true; n as usize];
        sieve[0] = false;
        sieve[1] = false;
        for i in 2..n as usize {
            if sieve[i] {
                for j in (2..).map(|j| j * i).take_while(|&j| j < n as usize) {
                    sieve[j] = false;
                }
            }
        }
        sieve.iter().filter(|&b| *b).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::count_primes(10));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::count_primes(0));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::count_primes(1));
    }
}
