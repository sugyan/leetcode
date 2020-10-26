pub struct Solution {}

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut v: Vec<f64> = vec![0.0; 100];
        v[0] = poured as f64;
        for i in 1..=query_row as usize {
            for j in (0..=i).rev() {
                v[j] = ((v[j] - 1.0) / 2.0).max(0.0);
                v[j + 1] += v[j];
            }
        }
        v[query_glass as usize].min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(0.0, Solution::champagne_tower(1, 1, 1));
    }

    #[test]
    fn example_2() {
        assert_eq!(0.5, Solution::champagne_tower(2, 1, 1));
    }

    #[test]
    fn example_3() {
        assert_eq!(1.0, Solution::champagne_tower(100000009, 33, 17));
    }
}
