pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut sum = 0;
        let mut answer = 0;
        for &num in nums.iter().chain(std::iter::once(&n)) {
            while num as i64 > sum + 1 {
                answer += 1;
                sum = sum * 2 + 1;
                if sum >= n as i64 {
                    return answer;
                }
            }
            sum += num as i64;
            if sum >= n as i64 {
                return answer;
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::min_patches(vec![1, 3], 6));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::min_patches(vec![1, 5, 10], 20));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::min_patches(vec![1, 2, 2], 5));
    }

    #[test]
    fn tle_1() {
        assert_eq!(28, Solution::min_patches(vec![1, 2, 31, 33], 2147483647));
    }

    #[test]
    fn wa_1() {
        assert_eq!(
            2,
            Solution::min_patches(
                vec![1, 7, 21, 31, 34, 37, 40, 43, 49, 87, 90, 92, 93, 98, 99],
                12
            )
        );
    }

    #[test]
    fn wa_2() {
        assert_eq!(
            1,
            Solution::min_patches(
                vec![1, 2, 2, 6, 34, 38, 41, 44, 47, 47, 56, 59, 62, 73, 77, 83, 87, 89, 94],
                20
            )
        );
    }
}
