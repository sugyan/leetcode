use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = version1
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        let v2 = version2
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        for i in 0..v1.len().max(v2.len()) {
            match v1.get(i).unwrap_or(&0).cmp(v2.get(i).unwrap_or(&0)) {
                Ordering::Less => return -1,
                Ordering::Equal => {}
                Ordering::Greater => return 1,
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            0,
            Solution::compare_version(String::from("1.01"), String::from("1.001"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::compare_version(String::from("1.0"), String::from("1.0.0"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            -1,
            Solution::compare_version(String::from("0.1"), String::from("1.1"))
        );
    }
}
