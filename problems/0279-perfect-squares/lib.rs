use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut hs: HashSet<i32> = HashSet::new();
        hs.insert(0);
        let v: Vec<i32> = (1..=(n as f32).sqrt() as i32).map(|m| m * m).collect();
        for i in 1.. {
            let mut add: Vec<i32> = Vec::new();
            for e in hs.iter() {
                for m in v.iter() {
                    if e + m == n {
                        return i;
                    }
                    add.push(e + m);
                }
            }
            hs.extend(add);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::num_squares(13));
    }
}
