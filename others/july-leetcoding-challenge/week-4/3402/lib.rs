pub struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        while num >= 10 {
            let mut m = 0;
            while num > 0 {
                m += num % 10;
                num /= 10;
            }
            num = m;
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::add_digits(38));
    }
}
