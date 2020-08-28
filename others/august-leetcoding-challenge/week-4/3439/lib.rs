use rand::Rng;

/**
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */
fn rand7() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, 8)
}

pub struct Solution {}

impl Solution {
    pub fn rand10() -> i32 {
        (0..10).fold(0, |acc, _| acc + rand7()) % 10 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut counts = [0; 10];
        for _ in 0..10_000 {
            let ret: i32 = Solution::rand10();
            assert!(1 <= ret && ret <= 10);
            counts[ret as usize - 1] += 1;
        }
        for &count in counts.iter() {
            assert!(900 <= count && count <= 1100);
        }
    }
}
