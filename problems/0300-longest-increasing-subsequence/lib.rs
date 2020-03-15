pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut v: Vec<i32> = Vec::with_capacity(nums.len());
        for num in nums.iter() {
            if let Some(i) = v.binary_search(num).err() {
                if i == v.len() {
                    v.push(*num)
                } else {
                    v[i] = *num
                }
            }
        }
        v.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }
}
