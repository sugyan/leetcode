pub struct Solution;

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let less = |mid: i32| {
            let mut sum = 0;
            for i in 1..=m {
                sum += (mid / i).min(n);
                if sum >= k {
                    return false;
                }
            }
            true
        };
        let (mut lo, mut hi) = (1, m * n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if less(mid) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::find_kth_number(3, 3, 5));
    }

    #[test]
    fn example_2() {
        assert_eq!(6, Solution::find_kth_number(2, 3, 6));
    }
}
