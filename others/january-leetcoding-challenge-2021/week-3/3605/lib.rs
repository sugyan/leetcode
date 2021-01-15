pub struct Solution;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut v = vec![0; n as usize + 1];
        v[1] = 1;
        let mut answer = 1;
        for i in 2..v.len() {
            v[i] = if i % 2 == 0 {
                v[i / 2]
            } else {
                v[i / 2] + v[i / 2 + 1]
            };
            answer = std::cmp::max(answer, v[i]);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::get_maximum_generated(7));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::get_maximum_generated(2));
    }

    #[test]
    fn example_3() {
        assert_eq!(2, Solution::get_maximum_generated(3));
    }
}
