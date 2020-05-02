pub struct Solution {}

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut d = [false; 256];
        for c in j.chars() {
            d[(c as u8) as usize] = true;
        }
        s.chars().filter(|c| d[(*c as u8) as usize]).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string())
        );
    }
}
