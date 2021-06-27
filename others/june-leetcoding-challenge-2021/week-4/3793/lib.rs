pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut v = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                v[i] = v[i].max(v[i - 1] + 1);
            }
        }
        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                v[i] = v[i].max(v[i + 1] + 1);
            }
        }
        v.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::candy(vec![1, 0, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::candy(vec![1, 2, 2]));
    }
}
