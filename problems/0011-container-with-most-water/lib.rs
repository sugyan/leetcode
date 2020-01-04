pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let mut answer = std::cmp::min(height[0], height[r]) * (r - l) as i32;
        while l < r {
            if height[l] < height[r] {
                let h = height[l];
                while height[l] <= h && l < r {
                    l += 1;
                }
            } else {
                let h = height[r];
                while height[r] <= h && l < r {
                    r -= 1;
                }
            }
            answer = std::cmp::max(answer, std::cmp::min(height[l], height[r]) * (r - l) as i32);
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
}
