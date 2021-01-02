pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        heights.push(0);
        let mut stack = Vec::new();
        let mut answer = 0;
        for i in 0..heights.len() {
            while !stack.is_empty() {
                if let Some(&last) = stack.last() {
                    if heights[i] >= heights[last] {
                        break;
                    }
                    if let Some(p) = stack.pop() {
                        let w = i as i32
                            - 1
                            - if let Some(&l) = stack.last() {
                                l as i32
                            } else {
                                -1
                            };
                        answer = std::cmp::max(answer, heights[p] * w);
                    }
                }
            }
            stack.push(i);
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
