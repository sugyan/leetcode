use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut banned = vec![false; 10_000];
        deadends
            .iter()
            .filter_map(|d| d.parse::<usize>().ok())
            .for_each(|i| banned[i] = true);
        let target = target.parse::<usize>().unwrap();
        let mut vd = VecDeque::new();
        if !banned[0] {
            vd.push_back((0, 0));
        }
        while let Some((state, moves)) = vd.pop_front() {
            if state == target {
                return moves;
            }
            for i in &[1, 10, 100, 1000] {
                let w = (state / i) % 10;
                for &j in &[
                    (state - i * w) + i * ((w + 1) % 10),
                    (state - i * w) + i * ((w + 9) % 10),
                ] {
                    if !banned[j] {
                        banned[j] = true;
                        vd.push_back((j, moves + 1));
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            6,
            Solution::open_lock(
                vec!["0201", "0101", "0102", "1212", "2002"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                String::from("0202")
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::open_lock(
                vec!["8888"].into_iter().map(String::from).collect(),
                String::from("0009")
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            -1,
            Solution::open_lock(
                vec!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                String::from("8888")
            )
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            -1,
            Solution::open_lock(
                vec!["0000"].into_iter().map(String::from).collect(),
                String::from("8888")
            )
        );
    }
}
