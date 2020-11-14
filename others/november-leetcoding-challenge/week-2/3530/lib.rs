pub struct Solution {}

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        if buckets == 1 {
            return 0;
        }
        let n = minutes_to_test / minutes_to_die;
        let can_figure_out = |m: i32| -> bool {
            let mut s = 1;
            for _ in 0..m {
                s *= n + 1;
                if s >= buckets {
                    return true;
                }
            }
            false
        };
        let (mut l, mut r) = (1, buckets);
        while l < r {
            let m = l + (r - l) / 2;
            if can_figure_out(m) {
                r = m
            } else {
                l = m + 1
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::poor_pigs(1000, 15, 60));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::poor_pigs(4, 15, 15));
    }
}
