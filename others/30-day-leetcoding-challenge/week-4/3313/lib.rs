use std::collections::{HashMap, VecDeque};

pub struct FirstUnique {
    vd: VecDeque<i32>,
    hm: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FirstUnique {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut vd: VecDeque<i32> = VecDeque::with_capacity(nums.len());
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for num in nums.iter() {
            vd.push_back(*num);
            *hm.entry(*num).or_insert(0) += 1;
        }
        FirstUnique { vd, hm }
    }

    pub fn show_first_unique(&mut self) -> i32 {
        while let Some(f) = self.vd.front() {
            if let Some(v) = self.hm.get(f) {
                if *v == 1 {
                    return *f;
                }
            }
            self.vd.pop_front();
        }
        -1
    }

    pub fn add(&mut self, value: i32) {
        self.vd.push_back(value);
        *self.hm.entry(value).or_insert(0) += 1;
    }
}

/**
 * Your FirstUnique object will be instantiated and called as such:
 * let obj = FirstUnique::new(nums);
 * let ret_1: i32 = obj.show_first_unique();
 * obj.add(value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = FirstUnique::new(vec![2, 3, 5]);
        assert_eq!(2, obj.show_first_unique());
        obj.add(5);
        assert_eq!(2, obj.show_first_unique());
        obj.add(2);
        assert_eq!(3, obj.show_first_unique());
        obj.add(3);
        assert_eq!(-1, obj.show_first_unique());
    }

    #[test]
    fn example_2() {
        let mut obj = FirstUnique::new(vec![7, 7, 7, 7, 7, 7]);
        assert_eq!(-1, obj.show_first_unique());
        obj.add(7);
        obj.add(3);
        obj.add(3);
        obj.add(7);
        obj.add(17);
        assert_eq!(17, obj.show_first_unique());
    }

    #[test]
    fn example_3() {
        let mut obj = FirstUnique::new(vec![809]);
        assert_eq!(809, obj.show_first_unique());
        obj.add(809);
        assert_eq!(-1, obj.show_first_unique());
    }
}
