use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Solution {
    rects: Vec<Vec<i32>>,
    btm: BTreeMap<i32, usize>,
    s: i32,
    tr: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut btm: BTreeMap<i32, usize> = BTreeMap::new();
        let mut s = 0;
        for (i, rect) in rects.iter().enumerate() {
            s += (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
            btm.insert(s - 1, i);
        }
        Self {
            rects,
            btm,
            s,
            ..Default::default()
        }
    }
    pub fn pick(&mut self) -> Vec<i32> {
        let &i = self
            .btm
            .range(self.tr.gen_range(0, self.s)..)
            .next()
            .unwrap()
            .1;
        let rect = &self.rects[i];
        vec![
            self.tr.gen_range(rect[0], rect[2] + 1),
            self.tr.gen_range(rect[1], rect[3] + 1),
        ]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = Solution::new(vec![vec![1, 1, 5, 5]]);
        for _ in 0..10_000 {
            let ret: Vec<i32> = obj.pick();
            assert!(ret[0] >= 1 && ret[0] <= 5);
            assert!(ret[1] >= 1 && ret[1] <= 5);
        }
    }

    #[test]
    fn example_2() {
        let mut obj = Solution::new(vec![vec![-2, -2, -1, -1], vec![1, 0, 3, 0]]);
        for _ in 0..10_000 {
            let ret: Vec<i32> = obj.pick();
            assert!((ret[0] >= -2 && ret[0] <= -1) || (ret[0] >= 1 && ret[0] <= 3));
            assert!((ret[1] >= -2 && ret[1] <= -1) || ret[1] == 0);
        }
    }
}
