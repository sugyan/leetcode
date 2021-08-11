use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut hm = HashMap::new();
        for &a in &arr {
            *hm.entry(a).or_insert(0) += 1;
        }
        let mut v = hm.keys().cloned().collect::<Vec<_>>();
        v.sort_unstable_by_key(|k| k.abs());
        for k in &v {
            let n1 = *hm.get(k).unwrap();
            if n1 > 0 {
                if let Some(n2) = hm.get_mut(&(k * 2)) {
                    *n2 -= n1;
                    if *n2 < 0 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(false, Solution::can_reorder_doubled(vec![3, 1, 3, 6]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::can_reorder_doubled(vec![2, 1, 2, 6]));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::can_reorder_doubled(vec![4, -2, 2, -4]));
    }

    #[test]
    fn example_4() {
        assert_eq!(
            false,
            Solution::can_reorder_doubled(vec![1, 2, 4, 16, 8, 4])
        );
    }
}
