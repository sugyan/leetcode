use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut bh: BinaryHeap<Reverse<(i32, &Vec<i32>)>> = BinaryHeap::with_capacity(points.len());
        for point in points.iter() {
            bh.push(Reverse((point[0] * point[0] + point[1] * point[1], point)));
        }
        let mut answer: Vec<Vec<i32>> = Vec::new();
        for _ in 0..k {
            answer.push((bh.pop().unwrap().0).1.clone());
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret: Vec<Vec<i32>> = Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1);
        ret.sort();
        assert_eq!(vec![vec![-2, 2]], ret);
    }

    #[test]
    fn example_2() {
        let mut ret: Vec<Vec<i32>> =
            Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
        ret.sort();
        assert_eq!(vec![vec![-2, 4], vec![3, 3]], ret);
    }
}
