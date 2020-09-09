pub struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<u32> = version1.split('.').map(|s| s.parse().unwrap()).collect();
        let v2: Vec<u32> = version2.split('.').map(|s| s.parse().unwrap()).collect();
        for i in 0..std::cmp::max(v1.len(), v2.len()) {
            let n1 = v1.get(i).unwrap_or(&0);
            let n2 = v2.get(i).unwrap_or(&0);
            match n1.cmp(&n2) {
                std::cmp::Ordering::Less => return -1,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => return 1,
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
            -1,
            Solution::compare_version(String::from("0.1"), String::from("1.1"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::compare_version(String::from("1.0.1"), String::from("1"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            -1,
            Solution::compare_version(String::from("7.5.2.4"), String::from("7.5.3"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            0,
            Solution::compare_version(String::from("1.01"), String::from("1.001"))
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            0,
            Solution::compare_version(String::from("1.0"), String::from("1.0.0"))
        );
    }
}
