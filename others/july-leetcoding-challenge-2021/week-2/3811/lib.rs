pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut dict = vec![None; 128];
        for (&s, t) in s.as_bytes().iter().zip(t.as_bytes()) {
            dict[s as usize] = match dict[s as usize].take() {
                Some(u) if u != t => return false,
                _ => Some(t),
            }
        }
        dict.iter()
            .filter_map(|&o| o)
            .fold([0; 128], |mut acc, &x| {
                acc[x as usize] += 1;
                acc
            })
            .iter()
            .all(|&c| c < 2)
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
