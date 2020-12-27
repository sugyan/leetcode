use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut same_values = HashMap::new();
        arr.iter()
            .enumerate()
            .for_each(|(i, &n)| same_values.entry(n).or_insert_with(Vec::new).push(i));
        let mut v = vec![false; arr.len()];
        let mut targets = vec![0];
        for jumps in 0.. {
            let mut next = Vec::new();
            for &i in targets.iter() {
                if i == arr.len() - 1 {
                    return jumps;
                }
                if v[i] {
                    continue;
                }
                v[i] = true;
                if let Some(indices) = same_values.get(&arr[i]) {
                    next.extend(indices);
                }
                same_values.remove(&arr[i]);
                if i > 0 {
                    next.push(i - 1);
                }
                if i < arr.len() {
                    next.push(i + 1);
                }
            }
            targets = next;
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::min_jumps(vec![7]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]));
    }

    #[test]
    fn example_4() {
        assert_eq!(2, Solution::min_jumps(vec![6, 1, 9]));
    }

    #[test]
    fn example_5() {
        assert_eq!(
            3,
            Solution::min_jumps(vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13])
        );
    }
}
