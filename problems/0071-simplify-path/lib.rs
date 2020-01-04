pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut v: Vec<&str> = Vec::new();
        for directory in path.split('/').filter(|s| !s.is_empty()) {
            match directory {
                "." => {}
                ".." => {
                    if !v.is_empty() {
                        v.pop();
                    }
                }
                _ => v.push(directory),
            }
        }
        "/".to_string() + &v.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("/home", Solution::simplify_path("/home/".to_string()));
    }

    #[test]
    fn example_2() {
        assert_eq!("/", Solution::simplify_path("/../".to_string()));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "/home/foo",
            Solution::simplify_path("/home//foo/".to_string())
        );
    }

    #[test]
    fn example_4() {
        assert_eq!("/c", Solution::simplify_path("/a/./b/../../c/".to_string()));
    }

    #[test]
    fn example_5() {
        assert_eq!(
            "/c",
            Solution::simplify_path("/a/../../b/../c//.//".to_string())
        );
    }

    #[test]
    fn example_6() {
        assert_eq!(
            "/a/b/c",
            Solution::simplify_path("/a//b////c/d//././/..".to_string())
        );
    }
}
