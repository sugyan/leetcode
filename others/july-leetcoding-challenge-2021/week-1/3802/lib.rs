pub struct Solution;

const MOD: u32 = 1_000_000_007;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        ((1..n)
            .fold([1; 5], |v, _| {
                [
                    (v[1] + v[2] + v[4]) % MOD,
                    (v[0] + v[2]) % MOD,
                    (v[1] + v[3]) % MOD,
                    (v[2]) % MOD,
                    (v[2] + v[3]) % MOD,
                ]
            })
            .iter()
            .sum::<u32>()
            % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::count_vowel_permutation(1));
    }

    #[test]
    fn example_2() {
        assert_eq!(10, Solution::count_vowel_permutation(2));
    }

    #[test]
    fn example_3() {
        assert_eq!(68, Solution::count_vowel_permutation(5));
    }
}
