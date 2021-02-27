pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let negative = (dividend < 0) != (divisor < 0);
        let (mut dividend, mut divisor) = (i64::from(dividend).abs(), i64::from(divisor).abs());
        let mut ret = 0_i64;
        let mut n = 1;
        while (divisor << 1) <= dividend {
            divisor <<= 1;
            n <<= 1;
        }
        while n > 0 {
            if dividend >= divisor {
                ret += n;
                dividend -= divisor;
            }
            divisor >>= 1;
            n >>= 1;
        }
        (if negative {
            -ret
        } else {
            ret.min(i64::from(std::i32::MAX))
        }) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::divide(10, 3));
    }

    #[test]
    fn example_2() {
        assert_eq!(-2, Solution::divide(7, -3));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::divide(0, 1));
    }

    #[test]
    fn example_4() {
        assert_eq!(1, Solution::divide(1, 1));
    }
}
