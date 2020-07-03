pub struct Solution {}

impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut cells: Vec<i32> = cells;
        for _ in 0..(n - 1) % 14 + 1 {
            cells = (0..cells.len())
                .map(|i| {
                    if i > 0 && i < cells.len() - 1 && cells[i - 1] == cells[i + 1] {
                        1
                    } else {
                        0
                    }
                })
                .collect();
        }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![0, 0, 1, 1, 0, 0, 0, 0],
            Solution::prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![0, 0, 1, 1, 1, 1, 1, 0],
            Solution::prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000)
        );
    }
}
