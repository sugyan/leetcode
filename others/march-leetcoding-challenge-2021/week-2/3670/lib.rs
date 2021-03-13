use std::collections::HashMap;

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut hm = arr.iter().map(|&n| (n, 1)).collect::<HashMap<_, _>>();
        let mut arr = arr;
        arr.sort_unstable();
        for i in 1..arr.len() {
            for j in 0..i {
                if arr[i] % arr[j] == 0 {
                    if let Some(&v) = hm.get(&(arr[i] / arr[j])) {
                        let vj = *hm.get_mut(&arr[j]).unwrap();
                        if let Some(vi) = hm.get_mut(&arr[i]) {
                            *vi = (*vi + vj * v) % MOD
                        }
                    }
                }
            }
        }
        (hm.values().sum::<i64>() % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::num_factored_binary_trees(vec![2, 4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(7, Solution::num_factored_binary_trees(vec![2, 4, 5, 10]));
    }
}
