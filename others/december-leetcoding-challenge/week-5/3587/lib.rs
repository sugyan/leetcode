pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if heights.is_empty() {
            return 0;
        }
        let mut lr = vec![(0, 0); heights.len()];
        lr[0].0 = -1;
        for i in 1..heights.len() {
            let mut p = i as i32 - 1;
            while p >= 0 && heights[p as usize] >= heights[i] {
                p = lr[p as usize].0;
            }
            lr[i].0 = p;
        }
        lr[heights.len() - 1].1 = heights.len() as i32;
        for i in (0..heights.len() - 1).rev() {
            let mut p = i as i32 + 1;
            while (p as usize) < heights.len() && heights[p as usize] >= heights[i] {
                p = lr[p as usize].1;
            }
            lr[i].1 = p;
        }
        let mut answer = 0;
        for (i, &(l, r)) in lr.iter().enumerate() {
            answer = std::cmp::max(answer, heights[i] * (r - l - 1));
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
    }
}
