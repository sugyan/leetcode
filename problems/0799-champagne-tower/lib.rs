pub struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut v: Vec<f64> = vec![0.0; query_row as usize + 2];
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
        assert!((0.0 - Solution::champagne_tower(1, 1, 1)).abs() < std::f64::EPSILON);
    }

    #[test]
    fn example_2() {
        assert!((0.5 - Solution::champagne_tower(2, 1, 1)).abs() < std::f64::EPSILON);
    }

    #[test]
    fn example_3() {
        assert!((1.0 - Solution::champagne_tower(100_000_009, 33, 17)).abs() < std::f64::EPSILON);
    }
}
