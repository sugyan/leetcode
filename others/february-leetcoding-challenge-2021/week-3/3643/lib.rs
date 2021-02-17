pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut answer = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            answer = std::cmp::max(answer, std::cmp::min(height[l], height[r]) * (r - l) as i32);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
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
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::max_area(vec![1, 1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(16, Solution::max_area(vec![4, 3, 2, 1, 4]));
    }

    #[test]
    fn example_4() {
        assert_eq!(2, Solution::max_area(vec![1, 2, 1]));
    }
}
