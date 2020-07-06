pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits: Vec<i32> = digits;
        let mut idx = digits.len() - 1;
        loop {
            if digits[idx] == 9 {
                digits[idx] = 0;
            } else {
                digits[idx] += 1;
                return digits;
            };
            if idx > 0 {
                idx -= 1;
            } else {
                digits.insert(0, 1);
                return digits;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
    }
}
