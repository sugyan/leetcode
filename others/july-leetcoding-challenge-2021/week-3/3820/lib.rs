use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

pub struct Solution {
    nums: Vec<i32>,
    prev: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut prev = nums.clone();
        let mut rng = rand::thread_rng();
        prev.shuffle(&mut rng);
        Self { nums, prev, rng }
    }

    /** Resets the array to its original configuration and return it. */
    pub fn reset(&mut self) -> Vec<i32> {
        self.prev.shuffle(&mut self.rng);
        self.nums.clone()
    }

    /** Returns a random shuffling of the array. */
    pub fn shuffle(&mut self) -> Vec<i32> {
        let len = self.prev.len();
        if let Some(i) = self.prev.windows(2).rev().position(|w| w[0] < w[1]) {
            let m = self.prev[len - 2 - i];
            if let Some(j) = (0..=i).find(|&j| self.prev[len - 1 - j] > m) {
                self.prev.swap(len - 2 - i, len - 1 - j);
                self.prev[len - 1 - i..].reverse();
            }
        } else {
            self.prev.reverse();
        }
        self.prev.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = Solution::new(vec![1, 2, 3]);
        // assert_eq!(vec![3, 1, 2], obj.shuffle());
        assert_eq!(vec![1, 2, 3], obj.reset());
        // assert_eq!(vec![1, 3, 2], obj.shuffle());
    }
}
