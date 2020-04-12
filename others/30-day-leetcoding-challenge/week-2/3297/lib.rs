use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut bh: BinaryHeap<i32> = BinaryHeap::from(stones);
        while bh.len() > 1 {
            let s1 = bh.pop().unwrap();
            let s2 = bh.pop().unwrap();
            if s1 != s2 {
                bh.push((s1 - s2).abs());
            }
        }
        if let Some(s) = bh.pop() {
            s
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
    }
}
