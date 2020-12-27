use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return 0;
        }
        let mut same_values: HashMap<i32, Vec<usize>> = HashMap::new();
        arr.iter()
            .enumerate()
            .for_each(|(i, &n)| same_values.entry(n).or_insert_with(Vec::new).push(i));
        let mut mins: Vec<usize> = vec![0; arr.len()];
        let mut nodes: HashSet<usize> = HashSet::new();
        nodes.insert(0);
        for jumps in 1.. {
            let mut next: HashSet<usize> = HashSet::new();
            for &i in nodes.iter() {
                let mut v: Vec<usize> = Vec::new();
                if let Some(indices) = same_values.get(&arr[i]) {
                    v.extend(indices);
                }
                if i > 0 {
                    v.push(i - 1)
                }
                if i < arr.len() - 1 {
                    v.push(i + 1);
                }
                for &dst in v.iter() {
                    if dst == arr.len() - 1 {
                        return jumps as i32;
                    }
                    if mins[dst] == 0 {
                        mins[dst] = jumps;
                    }
                    next.insert(dst);
                }
            }
            nodes = next;
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
