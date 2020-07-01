pub struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        if n < 0 {
            return 0;
        }
        let n = n as i64;
        let (mut l, mut r) = (0, n);
        while l < r {
            let m = l + (r - l) / 2;
            if n >= (m + 1) * (m + 2) / 2 {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::arrange_coins(5));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::arrange_coins(8));
    }
}
