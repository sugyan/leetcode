use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
        for p in prerequisites.iter() {
            if let Some(v) = hm.get_mut(&p[0]) {
                v.push(p[1]);
            } else {
                hm.insert(p[0], vec![p[1]]);
            }
        }
        let mut memo: HashSet<i32> = HashSet::new();
        for i in 0..num_courses {
            let mut hs: HashSet<i32> = HashSet::new();
            hs.insert(i);
            if !Solution::has_no_cycle(&hm, i, &mut hs, &mut memo) {
                return false;
            }
        }
        true
    }
    fn has_no_cycle(
        hm: &HashMap<i32, Vec<i32>>,
        src: i32,
        hs: &mut HashSet<i32>,
        memo: &mut HashSet<i32>,
    ) -> bool {
        if memo.contains(&src) {
            return true;
        }
        if let Some(v) = hm.get(&src) {
            for dst in v.iter() {
                if hs.contains(&dst) {
                    return false;
                }
                hs.insert(*dst);
                if !Solution::has_no_cycle(hm, *dst, hs, memo) {
                    return false;
                }
                hs.remove(dst);
            }
        } else {
            memo.extend(hs.iter());
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}
