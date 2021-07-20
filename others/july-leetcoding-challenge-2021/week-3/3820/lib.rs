use rand::{rngs::ThreadRng, Rng};

pub struct Solution {
    nums: Vec<i32>,
    arr: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        let arr = nums.clone();
        let rng = rand::thread_rng();
        Self { nums, arr, rng }
    }

    /** Resets the array to its original configuration and return it. */
    pub fn reset(&mut self) -> Vec<i32> {
        self.arr = self.nums.clone();
        self.arr.clone()
    }

    /** Returns a random shuffling of the array. */
    pub fn shuffle(&mut self) -> Vec<i32> {
        for i in (0..self.arr.len()).rev() {
            let j = self.rng.gen_range(0, i + 1);
            self.arr.swap(i, j);
        }
        self.arr.clone()
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
