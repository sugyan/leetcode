pub struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut iter = preorder.split(',').map(|s| s != "#");
        Self::dfs(&mut iter) && iter.next().is_none()
    }
    fn dfs(iter: &mut impl std::iter::Iterator<Item = bool>) -> bool {
        if let Some(b) = iter.next() {
            if b {
                if !Self::dfs(iter) {
                    return false;
                }
                if !Self::dfs(iter) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::is_valid_serialization(String::from("9,3,4,#,#,1,#,#,2,#,6,#,#"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_valid_serialization(String::from("1,#")));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::is_valid_serialization(String::from("9,#,#,1"))
        );
    }
}
