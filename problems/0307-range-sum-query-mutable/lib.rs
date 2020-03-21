pub struct NumArray {
    segtree: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut segtree: Vec<i32> = vec![0; nums.len() * 2];
        for i in 0..nums.len() {
            segtree[nums.len() + i] = nums[i];
        }
        for i in (0..nums.len()).rev() {
            segtree[i] = segtree[i * 2] + segtree[i * 2 + 1];
        }
        println!("{:?}", segtree);
        NumArray { segtree }
    }

    pub fn update(&mut self, i: i32, val: i32) {
        let mut i = i as usize + self.segtree.len() / 2;
        self.segtree[i] = val;
        while i > 0 {
            let j = if i % 2 == 0 { i + 1 } else { i - 1 };
            self.segtree[i / 2] = self.segtree[i] + self.segtree[j];
            i /= 2;
        }
    }

    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        let (mut l, mut r) = (i as usize, j as usize);
        l += self.segtree.len() / 2;
        r += self.segtree.len() / 2;
        let mut sum = 0;
        while l <= r {
            if l % 2 == 1 {
                sum += self.segtree[l];
                l += 1;
            }
            if r % 2 == 0 {
                sum += self.segtree[r];
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        sum
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
