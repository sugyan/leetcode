pub struct CombinationIterator {
    chars: Vec<char>,
    idx: i32,
    cl: u32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    pub fn new(characters: String, combination_length: i32) -> Self {
        Self {
            chars: characters.chars().collect(),
            idx: 1 << characters.len(),
            cl: combination_length as u32,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> String {
        loop {
            self.idx -= 1;
            if self.idx.count_ones() == self.cl {
                break;
            }
        }
        (0..self.chars.len())
            .filter(|i| self.idx & 1 << (self.chars.len() - i - 1) > 0)
            .map(|i| self.chars[i])
            .collect()
    }

    pub fn has_next(&self) -> bool {
        self.idx > (1 << self.cl) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = CombinationIterator::new("abc".to_string(), 2);
        assert_eq!("ab", obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!("ac", obj.next());
        assert_eq!(true, obj.has_next());
        assert_eq!("bc", obj.next());
        assert_eq!(false, obj.has_next());
    }
}
