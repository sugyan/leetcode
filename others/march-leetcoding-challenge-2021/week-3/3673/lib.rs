use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default)]
pub struct Codec {
    id: RefCell<usize>,
    table: RefCell<HashMap<String, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 */
impl Codec {
    pub fn new() -> Self {
        Default::default()
    }

    // Encodes a URL to a shortened URL.
    pub fn encode(&self, long_url: String) -> String {
        let id = self.new_id();
        let ret = String::from("http://tinyurl.com/") + &id;
        self.table.borrow_mut().insert(id, long_url);
        ret
    }

    // Decodes a shortened URL to its original URL.
    pub fn decode(&self, short_url: String) -> String {
        short_url
            .rsplit('/')
            .next()
            .map(|id| {
                self.table
                    .borrow()
                    .get(id)
                    .map(String::to_owned)
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    fn new_id(&self) -> String {
        let id = self.id.replace_with(|&mut old| old + 1);
        format!("{:X}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let obj = Codec::new();
        let s: String = obj.encode(String::from("https://leetcode.com/problems/design-tinyurl"));
        assert_eq!(
            "https://leetcode.com/problems/design-tinyurl",
            obj.decode(s)
        );
    }
}
