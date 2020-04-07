use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for n in arr.iter() {
            *(hm.entry(*n).or_insert(0)) += 1;
        }
        hm.iter()
            .filter(|e| hm.contains_key(&(e.0 + 1)))
            .map(|e| *e.1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::count_elements(vec![1, 2, 3]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]));
    }

    #[test]
    fn example_3() {
        assert_eq!(3, Solution::count_elements(vec![1, 3, 2, 3, 5, 0]));
    }

    #[test]
    fn example_4() {
        assert_eq!(2, Solution::count_elements(vec![1, 1, 2, 2]));
    }
}
