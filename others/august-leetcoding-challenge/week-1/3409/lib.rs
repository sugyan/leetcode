pub struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let v: Vec<char> = word.chars().collect();
        if v[0].is_lowercase() {
            return v.iter().all(|&c| c.is_lowercase());
        }
        if v.len() > 1 {
            let b = v[1].is_lowercase();
            return v.iter().skip(1).all(|&c| c.is_lowercase() == b);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::detect_capital_use("USA".to_string()));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::detect_capital_use("FlaG".to_string()));
    }
}
