pub struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut v = s.chars().collect::<Vec<_>>();
        let (mut lo, mut hi) = (0, v.len() - 1);
        while lo < hi {
            while lo < hi && !v[lo].is_alphabetic() {
                lo += 1;
            }
            while lo < hi && !v[hi].is_alphabetic() {
                hi -= 1;
            }
            v.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
        v.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "dc-ba",
            Solution::reverse_only_letters(String::from("ab-cd"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "j-Ih-gfE-dCba",
            Solution::reverse_only_letters(String::from("a-bC-dEf-ghIj"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "Qedo1ct-eeLg=ntse-T!",
            Solution::reverse_only_letters(String::from("Test1ng-Leet=code-Q!"))
        );
    }
}
