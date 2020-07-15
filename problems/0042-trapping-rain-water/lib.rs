pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut v: Vec<(i32, i32)> = vec![(0, 0); height.len()];
        {
            let mut max = 0;
            for i in 0..height.len() {
                max = std::cmp::max(max, height[i]);
                v[i].0 = max;
            }
        }
        {
            let mut max = 0;
            for i in (0..height.len()).rev() {
                max = std::cmp::max(max, height[i]);
                v[i].1 = max;
            }
        }
        (0..height.len())
            .map(|i| std::cmp::min(v[i].0, v[i].1) - height[i])
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }
}
