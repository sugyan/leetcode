use std::collections::HashMap;

#[derive(Default)]
pub struct FreqStack {
    v: Vec<i32>,
    hm: HashMap<i32, usize>,
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
        self.v.push(x);
        *self.hm.entry(x).or_insert(0) += 1;
    }

    pub fn pop(&mut self) -> i32 {
        if let Some(&max) = self.hm.values().max() {
            if let Some(i) = self.v.iter().rposition(|x| self.hm.get(x) == Some(&max)) {
                *self.hm.entry(self.v[i]).or_default() -= 1;
                return self.v.remove(i);
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
