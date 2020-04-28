use std::collections::HashSet;

pub struct FirstUnique {
    v: Vec<i32>,
    hs: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FirstUnique {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut hs: HashSet<i32> = HashSet::new();
        let mut v: Vec<i32> = Vec::new();
        for num in nums.iter() {
            if hs.contains(num) {
                if let Some(i) = v.iter().position(|&x| x == *num) {
                    v.remove(i);
                }
            } else {
                v.push(*num);
                hs.insert(*num);
            }
        }
        FirstUnique { v, hs }
    }

    pub fn show_first_unique(&self) -> i32 {
        if let Some(f) = self.v.first() {
            *f
        } else {
            -1
        }
    }

    pub fn add(&mut self, value: i32) {
        if self.hs.contains(&value) {
            if let Some(i) = self.v.iter().position(|&e| e == value) {
                self.v.remove(i);
            }
        } else {
            self.v.push(value);
            self.hs.insert(value);
        }
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
