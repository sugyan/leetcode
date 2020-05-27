use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut dmap: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for d in dislikes.iter() {
            dmap[d[0] as usize - 1].push(d[1] as usize - 1);
            dmap[d[1] as usize - 1].push(d[0] as usize - 1);
        }
        let mut v: Vec<Option<bool>> = vec![None; n as usize];
        for i in 0..n as usize {
            if v[i].is_some() {
                continue;
            }
            let mut vd: VecDeque<(usize, bool)> = VecDeque::new();
            vd.push_back((i, true));
            while let Some(last) = vd.pop_front() {
                if let Some(b) = v[last.0] {
                    if b != last.1 {
                        return false;
                    }
                    continue;
                }
                v[last.0] = Some(last.1);
                for j in dmap[last.0].iter() {
                    if v[*j].is_none() {
                        vd.push_back((*j, !last.1));
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::possible_bipartition(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]
            )
        );
    }
}
