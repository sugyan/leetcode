use std::collections::HashMap;

#[derive(Default)]
pub struct FreqStack {
    freq: HashMap<i32, usize>,
    group: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, x: i32) {
        let entry = self.freq.entry(x).or_insert(0);
        if *entry >= self.group.len() {
            self.group.push(Vec::new());
        }
        self.group[*entry].push(x);
        *entry += 1;
    }

    pub fn pop(&mut self) -> i32 {
        if let Some(maxfreq) = self.group.last_mut() {
            if let Some(ret) = maxfreq.pop() {
                *self.freq.entry(ret).or_default() -= 1;
                if maxfreq.is_empty() {
                    self.group.pop();
                }
                return ret;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = FreqStack::new();
        for &x in [5, 7, 5, 7, 4, 5].iter() {
            obj.push(x);
        }
        assert_eq!(5, obj.pop());
        assert_eq!(7, obj.pop());
        assert_eq!(5, obj.pop());
        assert_eq!(4, obj.pop());
    }
}
