pub struct Solution {}

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut m = 0;
        for i in 1..=k {
            m = (m * 10 + 1) % k;
            if m == 0 {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::smallest_repunit_div_by_k(1));
    }

    #[test]
    fn example_2() {
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(2));
    }

    #[test]
    fn example_3() {
        assert_eq!(3, Solution::smallest_repunit_div_by_k(3));
    }
}
