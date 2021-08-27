pub struct Solution;

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let is_subsequence = |s1: &str, s2: &str| -> bool {
            let mut s2 = s2.chars();
            for c1 in s1.chars() {
                loop {
                    if let Some(c2) = s2.next() {
                        if c2 == c1 {
                            break;
                        }
                    } else {
                        return false;
                    }
                }
            }
            true
        };
        let mut strs = strs;
        strs.sort_unstable_by_key(|s| s.len());
        for (i, s) in strs.iter().enumerate().rev() {
            if (0..strs.len())
                .filter(|&j| i != j)
                .all(|j| !is_subsequence(s, &strs[j]))
            {
                return s.len() as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::find_lu_slength(
                vec!["aba", "cdc", "eae"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            -1,
            Solution::find_lu_slength(
                vec!["aaa", "aaa", "aa"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }
}
