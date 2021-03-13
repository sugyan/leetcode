use std::collections::HashMap;

pub struct Solution;

const DIV: i64 = 1_000_000_007;

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut hm = arr
            .iter()
            .map(|&n| (i64::from(n), 1))
            .collect::<HashMap<_, _>>();
        let mut arr = arr.into_iter().map(i64::from).collect::<Vec<_>>();
        arr.sort_unstable();
        for i in 1..arr.len() {
            for j in (0..i).filter(|&j| arr[j] * arr[j] <= arr[i] && arr[i] % arr[j] == 0) {
                if let Some(&v) = hm.get(&(arr[i] / arr[j])) {
                    let n = (*hm.get(&arr[j]).unwrap()
                        * v
                        * if arr[j] * arr[j] != arr[i] { 2 } else { 1 })
                        % DIV;
                    *hm.entry(arr[i]).or_default() += n;
                }
            }
        }
        hm.values()
            .fold(0, |acc, &x| ((i64::from(acc) + x) % DIV) as i32)
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
