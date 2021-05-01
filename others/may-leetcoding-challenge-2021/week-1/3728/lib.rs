use std::collections::HashMap;

pub struct WordFilter {
    index_map: HashMap<String, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    pub fn new(words: Vec<String>) -> Self {
        let mut index_map = HashMap::new();
        for (i, word) in words.iter().enumerate() {
            for j in 0..word.len() {
                let prefix = &word[..=j];
                for k in 0..word.len() {
                    index_map.insert(
                        String::new() + prefix + "#" + &word[word.len() - 1 - k..],
                        i,
                    );
                }
            }
        }
        Self { index_map }
    }

    pub fn f(&self, prefix: String, suffix: String) -> i32 {
        if let Some(&i) = self
            .index_map
            .get(&(String::new() + &prefix + "#" + &suffix))
        {
            return i as i32;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let obj = WordFilter::new(vec![String::from("apple")]);
        assert_eq!(0, obj.f(String::from("a"), String::from("e")));
    }
}
