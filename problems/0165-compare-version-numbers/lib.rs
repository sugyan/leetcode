pub struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1: Vec<u32> = version1.split('.').map(|s| s.parse().unwrap()).collect();
        let mut v2: Vec<u32> = version2.split('.').map(|s| s.parse().unwrap()).collect();
        while v1.len() < v2.len() {
            v1.push(0);
        }
        while v2.len() < v1.len() {
            v2.push(0);
        }
        match v1.cmp(&v2) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            -1,
            Solution::compare_version("0.1".to_string(), "1.1".to_string())
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::compare_version("1.0.1".to_string(), "1".to_string())
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            -1,
            Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string())
        )
    }

    #[test]
    fn example_4() {
        assert_eq!(
            0,
            Solution::compare_version("1.01".to_string(), "1.001".to_string())
        )
    }

    #[test]
    fn example_5() {
        assert_eq!(
            0,
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string())
        )
    }
}
