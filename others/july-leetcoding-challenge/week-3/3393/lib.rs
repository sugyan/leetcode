use std::collections::{BinaryHeap, HashMap};

pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for num in nums {
            *hm.entry(num).or_insert(0) += 1;
        }
        let mut bh: BinaryHeap<(usize, i32)> = BinaryHeap::with_capacity(hm.len());
        for e in hm.iter() {
            bh.push((*e.1, *e.0))
        }
        let mut answer: Vec<i32> = Vec::with_capacity(k as usize);
        for _ in 0..k {
            if let Some(last) = bh.pop() {
                answer.push(last.1);
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
        assert_eq!(
            vec![1, 2],
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1], Solution::top_k_frequent(vec![1], 1));
    }
}
