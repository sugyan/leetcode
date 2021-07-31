pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut v = vec![i32::MAX; height.len()];
        {
            let mut max = 0;
            for (i, &h) in height.iter().enumerate() {
                max = max.max(h);
                v[i] = v[i].min(max);
            }
        }
        {
            let mut max = 0;
            for (i, &h) in height.iter().enumerate().rev() {
                max = max.max(h);
                v[i] = v[i].min(max);
            }
        }
        height.iter().zip(&v).map(|(h, m)| m - h).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(9, Solution::trap(vec![4, 2, 0, 3, 2, 5]));
    }
}
