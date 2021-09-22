pub struct Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let arr = arr
            .iter()
            .filter_map(|s| {
                let u = s.bytes().map(|u| 1 << (u - b'a')).sum::<u32>();
                if u.count_ones() == s.len() as u32 {
                    Some(u)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        (0..1 << arr.len())
            .filter_map(|i| {
                (0..arr.len())
                    .filter(|&j| i & 1 << j != 0)
                    .try_fold(0, |acc, j| {
                        if acc & arr[j] == 0 {
                            Some(acc | arr[j])
                        } else {
                            None
                        }
                    })
                    .map(|u| u.count_ones())
            })
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::max_length(
                vec!["un", "iq", "ue"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            6,
            Solution::max_length(
                vec!["cha", "r", "act", "ers"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            26,
            Solution::max_length(
                vec!["abcdefghijklmnopqrstuvwxyz"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }
}
