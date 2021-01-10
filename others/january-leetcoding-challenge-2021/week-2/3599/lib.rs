pub struct Solution;

const SIZE: usize = 100_001;
const DIV: i32 = 1_000_000_007;

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut bit = vec![0; SIZE];
        let mut answer = 0;
        for (i, &instruction) in instructions.iter().enumerate() {
            answer += std::cmp::min(
                Self::sum_bit(instruction - 1, &bit),
                i as i32 - Self::sum_bit(instruction, &bit),
            );
            answer %= DIV;
            Self::add_bit(instruction, &mut bit);
        }
        answer
    }
    fn add_bit(n: i32, bit: &mut [i32]) {
        let mut n = n;
        while n < SIZE as i32 {
            bit[n as usize] += 1;
            n += n & -n;
        }
    }
    fn sum_bit(n: i32, bit: &[i32]) -> i32 {
        let mut n = n;
        let mut ret = 0;
        while n > 0 {
            ret += bit[n as usize];
            n -= n & -n;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::create_sorted_array(vec![1, 5, 6, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            4,
            Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2])
        );
    }
}
