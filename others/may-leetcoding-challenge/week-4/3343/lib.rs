pub struct Solution {}

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![0; num as usize + 1];
        let mut n = 1;
        for i in 1..=num as usize {
            if i >= 1 << n {
                n += 1;
            }
            answer[i] = n - answer[(1 << n) - 1 - i];
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
