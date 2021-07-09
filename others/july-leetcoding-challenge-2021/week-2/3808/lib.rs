pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(Vec::new(), |mut v, x| {
                if let Err(i) = v.binary_search(x) {
                    if i < v.len() {
                        v[i] = *x
                    } else {
                        v.push(*x)
                    }
                };
                v
            })
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]));
    }
}
