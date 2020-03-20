pub struct NumArray {
    nums: Vec<i32>,
    sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut sum = nums.clone();
        for i in 1..sum.len() {
            sum[i] += sum[i - 1];
        }
        NumArray { nums, sum }
    }

    pub fn update(&mut self, i: i32, val: i32) {
        let diff = val - self.nums[i as usize];
        self.nums[i as usize] = val;
        for j in i as usize..self.sum.len() {
            self.sum[j] += diff;
        }
    }

    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        let mut ret = self.sum[j as usize];
        if i > 0 {
            ret -= self.sum[i as usize - 1];
        }
        ret
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(i, val);
 * let ret_2: i32 = obj.sum_range(i, j);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = NumArray::new(vec![1, 3, 5]);
        assert_eq!(9, obj.sum_range(0, 2));
        obj.update(1, 2);
        assert_eq!(8, obj.sum_range(0, 2));
    }
}
