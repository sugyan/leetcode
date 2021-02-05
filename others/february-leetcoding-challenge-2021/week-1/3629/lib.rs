pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for directory in path.split('/').filter(|&s| !s.is_empty()) {
            match directory {
                "." => {}
                ".." => {
                    stack.pop();
                }
                d => stack.push(d),
            }
        }
        String::from("/") + &stack.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("/home", Solution::simplify_path(String::from("/home/")));
    }

    #[test]
    fn example_2() {
        assert_eq!("/", Solution::simplify_path(String::from("/../")));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "/home/foo",
            Solution::simplify_path(String::from("/home//foo/"))
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            "/c",
            Solution::simplify_path(String::from("/a/./b/../../c/"))
        );
    }
}
