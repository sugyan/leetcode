pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 | 1 => n,
            2 => 1,
            _ => *(3..=n as usize)
                .fold(vec![0, 1, 1], |mut v, i| {
                    v.push(v[i - 1] + v[i - 2] + v[i - 3]);
                    v
                })
                .last()
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::tribonacci(4));
    }

    #[test]
    fn example_2() {
        assert_eq!(1389537, Solution::tribonacci(25));
    }
}
