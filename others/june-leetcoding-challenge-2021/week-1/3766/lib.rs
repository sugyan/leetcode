pub struct Solution;

const DIV: i64 = 1_000_000_007;

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut hc = horizontal_cuts;
        let mut vc = vertical_cuts;
        hc.extend(&[0, h]);
        vc.extend(&[0, w]);
        hc.sort_unstable();
        vc.sort_unstable();
        let hmax = hc.windows(2).map(|w| w[1] - w[0]).max().unwrap() as i64;
        let vmax = vc.windows(2).map(|w| w[1] - w[0]).max().unwrap() as i64;
        ((hmax * vmax) % DIV) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]));
    }

    #[test]
    fn example_2() {
        assert_eq!(6, Solution::max_area(5, 4, vec![3, 1], vec![1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(9, Solution::max_area(5, 4, vec![3], vec![3]));
    }
}
