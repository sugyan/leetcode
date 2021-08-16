pub struct NumArray {
    sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut sums = Vec::with_capacity(nums.len() + 1);
        sums.push(0);
        nums.iter()
            .enumerate()
            .for_each(|(i, num)| sums.push(sums[i] + num));
        Self { sums }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sums[right as usize + 1] - self.sums[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(1, obj.sum_range(0, 2));
        assert_eq!(-1, obj.sum_range(2, 5));
        assert_eq!(-3, obj.sum_range(0, 5));
    }
}
