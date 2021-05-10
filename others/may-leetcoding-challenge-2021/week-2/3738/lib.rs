pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        if n < 8 {
            return [0, 0, 0, 1, 2, 2, 3, 3][n];
        }
        let mut sieve = vec![0_u8; ((n - 1) >> 3) + 1];
        let inc = [6, 4, 2, 4, 2, 4, 6, 2];
        let mut i = 1;
        let mut answer = 3;
        for k in 0.. {
            i += inc[k & 7];
            if i >= n {
                break;
            }
            if sieve[i >> 3] & 1 << (i & 7) == 0 {
                answer += 1;
                let mut j = i * i;
                while j < n {
                    sieve[j >> 3] |= 1 << (j & 7);
                    j += i;
                }
            }
        }
        answer
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
