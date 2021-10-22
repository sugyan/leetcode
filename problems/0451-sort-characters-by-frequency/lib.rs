pub struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let c = s.bytes().fold([0; 128], |mut acc, u| {
            acc[u as usize] += 1;
            acc
        });
        let mut v = c.iter().enumerate().collect::<Vec<_>>();
        v.sort_by_cached_key(|(_, &n)| n);
        let mut answer = String::with_capacity(s.len());
        for &(i, &c) in v.iter().rev() {
            (0..c).for_each(|_| answer.push((i as u8) as char));
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(matches!(
            Solution::frequency_sort(String::from("tree")).as_str(),
            "eert" | "eetr"
        ));
    }

    #[test]
    fn example_2() {
        assert!(matches!(
            Solution::frequency_sort(String::from("cccaaa")).as_str(),
            "aaaccc" | "cccaaa"
        ));
    }

    #[test]
    fn example_3() {
        assert!(matches!(
            Solution::frequency_sort(String::from("Aabb")).as_str(),
            "bbAa" | "bbaA"
        ));
    }
}
