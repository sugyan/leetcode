use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a_sorted = a;
        let mut b_sorted = b.clone();
        a_sorted.sort_unstable();
        b_sorted.sort_unstable();
        let mut hm: HashMap<_, Vec<_>> = HashMap::new();
        let mut remain = Vec::new();
        let mut i = 0;
        for &n in &a_sorted {
            if n > b_sorted[i] {
                hm.entry(b_sorted[i]).or_default().push(n);
                i += 1;
            } else {
                remain.push(n);
            }
        }
        b.into_iter()
            .filter_map(|n| {
                hm.get_mut(&n)
                    .filter(|v| !v.is_empty())
                    .map_or_else(|| remain.pop(), |v| v.pop())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut a = vec![2, 7, 11, 15];
        let b = vec![1, 10, 4, 11];
        let mut ret = Solution::advantage_count(a.clone(), b.clone());
        assert_eq!(4, ret.iter().zip(b.iter()).filter(|(a, b)| a > b).count());
        ret.sort_unstable();
        a.sort_unstable();
        assert_eq!(ret, a);
    }

    #[test]
    fn example_2() {
        let mut a = vec![12, 24, 8, 32];
        let b = vec![13, 25, 32, 11];
        let mut ret = Solution::advantage_count(a.clone(), b.clone());
        assert_eq!(3, ret.iter().zip(b.iter()).filter(|(a, b)| a > b).count());
        ret.sort_unstable();
        a.sort_unstable();
        assert_eq!(ret, a);
    }
}
