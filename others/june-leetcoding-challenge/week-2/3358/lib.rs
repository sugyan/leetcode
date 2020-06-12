use rand::Rng;

#[derive(Default)]
pub struct RandomizedSet {
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }
    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    pub fn insert(&mut self, val: i32) -> bool {
        let (mut l, mut r) = (0, self.v.len());
        while l < r {
            let m = l + (r - l) / 2;
            if self.v[m] < val {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if l < self.v.len() && self.v[l] == val {
            false
        } else {
            self.v.insert(l, val);
            true
        }
    }
    /** Removes a value from the set. Returns true if the set contained the specified element. */
    pub fn remove(&mut self, val: i32) -> bool {
        let (mut l, mut r) = (0, self.v.len());
        while l < r {
            let m = l + (r - l) / 2;
            if self.v[m] < val {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if l < self.v.len() && self.v[l] == val {
            self.v.remove(l);
            true
        } else {
            false
        }
    }
    /** Get a random element from the set. */
    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        self.v[rng.gen_range(0, self.v.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = RandomizedSet::new();
        assert_eq!(true, obj.insert(1));
        assert_eq!(false, obj.remove(2));
        assert_eq!(true, obj.insert(2));
        let ret: i32 = obj.get_random();
        assert!(ret == 1 || ret == 2);
        assert_eq!(true, obj.remove(1));
        assert_eq!(false, obj.insert(2));
        assert_eq!(2, obj.get_random());
    }
}
