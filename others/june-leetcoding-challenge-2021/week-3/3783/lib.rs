pub struct NumArray {
    segtree: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut segtree = vec![0; nums.len() << 1];
        for (i, &num) in nums.iter().enumerate() {
            segtree[nums.len() + i] = num;
        }
        for i in (1..nums.len()).rev() {
            segtree[i] = segtree[i << 1] + segtree[(i << 1) + 1];
        }
        Self { segtree }
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let mut i = index as usize + (self.segtree.len() >> 1);
        self.segtree[i] = val;
        while i > 0 {
            self.segtree[i >> 1] = self.segtree[i] + self.segtree[i ^ 1];
            i >>= 1;
        }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (mut l, mut r) = (left as usize, right as usize);
        l += self.segtree.len() >> 1;
        r += self.segtree.len() >> 1;
        let mut sum = 0;
        while l <= r {
            if l & 1 == 1 {
                sum += self.segtree[l];
                l += 1;
            }
            if r & 1 == 0 {
                sum += self.segtree[r];
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        sum
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
