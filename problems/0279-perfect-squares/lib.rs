use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        fn helper(n: i32, hm: &mut HashMap<i32, i32>) -> i32 {
            if n == 0 {
                return 0;
            }
            if let Some(v) = hm.get(&n) {
                return *v;
            }
            let mut answer = std::i32::MAX;
            for i in 1.. {
                if i * i > n {
                    break;
                }
                answer = std::cmp::min(answer, 1 + helper(n - i * i, hm));
            }
            hm.insert(n, answer);
            answer
        }
        helper(n, &mut hm)
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
