pub struct Solution {}

impl Solution {
    pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
        let mut total = 0;
        let (mut maxsum, mut minsum) = (std::i32::MIN, std::i32::MAX);
        let (mut curmax, mut curmin) = (0, 0);
        for n in a.iter() {
            curmax = std::cmp::max(curmax + *n, *n);
            maxsum = std::cmp::max(maxsum, curmax);
            curmin = std::cmp::min(curmin + *n, *n);
            minsum = std::cmp::min(minsum, curmin);
            total += *n;
        }
        if maxsum > 0 {
            std::cmp::max(maxsum, total - minsum)
        } else {
            maxsum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(10, Solution::max_subarray_sum_circular(vec![5, -3, 5]));
    }

    #[test]
    fn example_3() {
        assert_eq!(4, Solution::max_subarray_sum_circular(vec![3, -1, 2, -1]));
    }

    #[test]
    fn example_4() {
        assert_eq!(3, Solution::max_subarray_sum_circular(vec![3, -2, 2, -3]));
    }

    #[test]
    fn example_5() {
        assert_eq!(-1, Solution::max_subarray_sum_circular(vec![-2, -3, -1]));
    }
}
