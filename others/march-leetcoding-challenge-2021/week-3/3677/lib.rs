pub struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut unlocked = vec![false; rooms.len()];
        unlocked[0] = true;
        let mut stack = vec![0];
        while let Some(next) = stack.pop() {
            for &i in &rooms[next as usize] {
                if !unlocked[i as usize] {
                    unlocked[i as usize] = true;
                    stack.push(i);
                }
            }
        }
        unlocked.iter().all(|&b| b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]])
        );
    }
}
