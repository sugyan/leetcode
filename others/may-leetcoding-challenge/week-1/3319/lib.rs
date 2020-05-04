pub struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut n = 1;
        while n < num {
            n = (n << 1) + 1;
        }
        n - num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::find_complement(5));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::find_complement(0));
    }
}
