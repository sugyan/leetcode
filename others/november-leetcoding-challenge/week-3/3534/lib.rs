pub struct Solution {}

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let (mut p, mut q) = (p, q);
        while p % 2 == 0 && q % 2 == 0 {
            p >>= 1;
            q >>= 1;
        }
        if p % 2 == 0 {
            2
        } else if q % 2 == 0 {
            0
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::mirror_reflection(2, 1));
    }
}
