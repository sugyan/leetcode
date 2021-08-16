pub struct NumArray {
    sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        Self {
            sums: nums
                .iter()
                .scan(0, |state, &x| {
                    *state += x;
                    Some(*state)
                })
                .collect(),
        }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sums[right as usize]
            - if left > 0 {
                self.sums[left as usize - 1]
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
        let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(1, obj.sum_range(0, 2));
        assert_eq!(-1, obj.sum_range(2, 5));
        assert_eq!(-3, obj.sum_range(0, 5));
    }
}
