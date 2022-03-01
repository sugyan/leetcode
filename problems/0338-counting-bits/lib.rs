pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut answer = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            answer[i] = answer[i >> 1] + (i & 1) as i32;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![0, 1, 1], Solution::count_bits(2));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::count_bits(5));
    }
}
