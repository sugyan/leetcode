pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (mut ds, mut dt) = (vec![0; 128], vec![0; 128]);
        for (i, (&s, &t)) in s.as_bytes().iter().zip(t.as_bytes()).enumerate() {
            if ds[s as usize] != dt[t as usize] {
                return false;
            }
            ds[s as usize] = i + 1;
            dt[t as usize] = i + 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::is_isomorphic(String::from("egg"), String::from("add"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::is_isomorphic(String::from("foo"), String::from("bar"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::is_isomorphic(String::from("paper"), String::from("title"))
        );
    }
}
