use rand::Rng;
use std::collections::BTreeMap;

pub struct Solution {
    btm: BTreeMap<i32, usize>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(w: Vec<i32>) -> Self {
        let mut btm: BTreeMap<i32, usize> = BTreeMap::new();
        let mut n = 0;
        for weight in w.iter() {
            n += *weight;
            btm.insert(n, btm.len());
        }
        Self { btm, n }
    }

    pub fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let m: i32 = rng.gen_range(0, self.n);
        if let Some(next) = self.btm.range(m + 1..).next() {
            *next.1 as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let obj = Solution::new(vec![1]);
        assert_eq!(0, obj.pick_index());
    }

    #[test]
    fn example_2() {
        let obj = Solution::new(vec![1, 3]);
        let mut ret: Vec<usize> = vec![0, 0];
        for _ in 0..10000 {
            ret[obj.pick_index() as usize] += 1;
        }
        assert!((4800..=5200).contains(&(ret[1] - ret[0])));
    }
}
