pub struct NumArray {
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut size = 1;
        while size <= nums.len() {
            size <<= 1;
        }
        let mut v = vec![0; size << 1];
        for (i, &num) in nums.iter().enumerate() {
            v[size + i] = num;
        }
        for i in (1..size).rev() {
            v[i] = v[i * 2] + v[i * 2 + 1];
        }
        Self { v }
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let offset = self.v.len() >> 1;
        let mut i = index as usize + offset;
        self.v[i] = val;
        while i > 0 {
            self.v[i / 2] = self.v[i / 2 * 2] + self.v[i / 2 * 2 + 1];
            i /= 2;
        }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let sum = |i: i32| -> i32 {
            if i < 0 {
                return 0;
            }
            let mut ret = 0;
            let mut j = 1;
            for k in (0..self.v.len().trailing_zeros() - 1).rev() {
                j <<= 1;
                if i >> k & 1 > 0 {
                    ret += self.v[j];
                    j += 1;
                }
            }
            ret + self.v[j]
        };
        sum(right) - sum(left - 1)
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
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
