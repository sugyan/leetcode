pub struct Solution;

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut strs = strs;
        strs.sort_unstable_by_key(|s| s.len());
        if let Some(i) = (0..strs.len()).rfind(|&i| {
            (0..strs.len()).all(|j| j == i || !Self::is_subsequence(&strs[i], &strs[j]))
        }) {
            strs[i].len() as i32
        } else {
            -1
        }
    }
    fn is_subsequence(s1: &str, s2: &str) -> bool {
        let mut s2 = s2.chars();
        s1.chars().all(|c1| s2.any(|c2| c2 == c1))
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
