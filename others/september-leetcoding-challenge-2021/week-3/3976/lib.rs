use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let v = arr.windows(2).map(|w| w[0].cmp(&w[1])).collect::<Vec<_>>();
        let mut max = 1;
        let mut len = 1;
        for i in 0..v.len() {
            len = if i == 0 {
                if v[0] == Ordering::Equal {
                    1
                } else {
                    2
                }
            } else {
                match (v[i], v[i - 1]) {
                    (Ordering::Less, Ordering::Greater) | (Ordering::Greater, Ordering::Less) => {
                        len + 1
                    }
                    (Ordering::Equal, _) => 1,
                    _ => 2,
                }
            };
            max = max.max(len);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            5,
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::max_turbulence_size(vec![4, 8, 12, 16]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::max_turbulence_size(vec![100]));
    }
}
