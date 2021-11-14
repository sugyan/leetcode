pub struct CombinationIterator {
    chars: Vec<char>,
    i: usize,
    combination_length: u32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    #[allow(non_snake_case)]
    pub fn new(characters: String, combinationLength: i32) -> Self {
        Self {
            chars: characters.chars().collect(),
            i: (1 << characters.len()) - 1,
            combination_length: combinationLength as u32,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> String {
        while self.i.count_ones() != self.combination_length {
            self.i -= 1;
        }
        let i = self.i;
        self.i -= 1;
        (0..self.chars.len())
            .filter_map(|j| {
                if i & 1 << (self.chars.len() - j - 1) > 0 {
                    Some(self.chars[j])
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn has_next(&self) -> bool {
        self.i >= 1 << self.combination_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = CombinationIterator::new(String::from("abc"), 2);
        assert_eq!("ab", obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!("ac", obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!("bc", obj.next());
        assert_eq!(false, obj.has_next());
    }
}
