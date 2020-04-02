pub struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        // 1 -> 1
        // 2 -> 4 -> 4 (16 -> 37 -> 58 -> 89 -> 145 -> 42 -> 20)
        // 3 -> 9 -> 4 (81 -> 65 -> 61 -> 37 -> 58 -> 89 -> 145 -> 42 -> 20)
        // 4 -> 4
        // 5 -> 4 (25 -> 29 -> 85 -> 89 -> 145 -> 42 -> 20)
        // 6 -> 4 (36 -> 45 -> 41 -> 17 -> 50 -> 25 -> 29 -> 85 -> 89 -> 145 -> 42 -> 20)
        // 7 -> 1 (49 -> 97 -> 130 -> 10)
        // 8 -> 4 (64 -> 52 -> 29 -> 85 -> 89 -> 145 -> 42 -> 20)
        // 9 -> 4
        let mut n = n;
        loop {
            let mut sum = 0;
            while n > 0 {
                let d = n % 10;
                sum += d * d;
                n /= 10;
            }
            match sum {
                1 => return true,
                4 => return false,
                _ => n = sum,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_happy(19));
    }
}
