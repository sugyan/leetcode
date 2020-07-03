use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut hm: HashMap<Vec<i32>, usize> = HashMap::new();
        let mut cells: Vec<i32> = cells;
        for i in 0.. {
            if i as i32 == n {
                break;
            }
            let mut new_cells = vec![0; cells.len()];
            for j in 1..cells.len() - 1 {
                if cells[j - 1] == cells[j + 1] {
                    new_cells[j] = 1;
                }
            }
            if let Some(j) = hm.get(&cells) {
                for e in hm.iter() {
                    if *e.1 == (n as usize - i) % (i - j) + j {
                        return e.0.clone();
                    }
                }
            } else {
                hm.insert(cells, i);
            }
            cells = new_cells;
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
