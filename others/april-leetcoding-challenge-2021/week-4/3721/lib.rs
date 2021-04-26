use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks = bricks;
        let mut bh = BinaryHeap::new();
        for i in 1..heights.len() {
            if heights[i] > heights[i - 1] {
                bh.push(Reverse(heights[i] - heights[i - 1]));
                if bh.len() > ladders as usize {
                    if let Some(Reverse(min)) = bh.pop() {
                        bricks -= min;
                        if bricks < 0 {
                            return i as i32 - 1;
                        }
                    }
                }
            }
        }
        heights.len() as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            7,
            Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(3, Solution::furthest_building(vec![14, 3, 19, 3], 17, 0));
    }
}
