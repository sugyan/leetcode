pub struct CombinationIterator {
    combinations: Vec<String>,
    idx: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    pub fn new(characters: String, combination_length: i32) -> Self {
        let mut combinations: Vec<String> = Vec::new();
        let mut v: Vec<usize> = Vec::new();
        let chars: Vec<char> = characters.chars().collect();
        CombinationIterator::dfs(
            &mut combinations,
            chars.len(),
            combination_length as usize,
            0,
            &mut v,
            &chars,
        );
        Self {
            combinations,
            idx: 0,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> String {
        let ret = self.combinations[self.idx].clone();
        self.idx += 1;
        ret
    }

    pub fn has_next(&self) -> bool {
        self.idx < self.combinations.len()
    }

    fn dfs(
        combinations: &mut Vec<String>,
        n: usize,
        k: usize,
        i: usize,
        v: &mut Vec<usize>,
        chars: &[char],
    ) {
        if v.len() == k {
            combinations.push(v.iter().map(|&i| chars[i]).collect());
            return;
        }
        for j in i..n {
            if n - j < k - v.len() {
                break;
            }
            v.push(j);
            CombinationIterator::dfs(combinations, n, k, j + 1, v, chars);
            v.pop();
        }
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
