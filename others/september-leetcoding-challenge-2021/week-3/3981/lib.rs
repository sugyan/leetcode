pub struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut grid = [[""; 3]; 3];
        for (i, m) in moves.iter().enumerate() {
            grid[m[0] as usize][m[1] as usize] = if i % 2 == 0 { "A" } else { "B" };
        }
        for v in &[
            [[0, 0], [0, 1], [0, 2]],
            [[1, 0], [1, 1], [1, 2]],
            [[2, 0], [2, 1], [2, 2]],
            [[0, 0], [1, 0], [2, 0]],
            [[0, 1], [1, 1], [2, 1]],
            [[0, 2], [1, 2], [2, 2]],
            [[0, 0], [1, 1], [2, 2]],
            [[0, 2], [1, 1], [2, 0]],
        ] {
            if grid[v[0][0]][v[0][1]] == grid[v[1][0]][v[1][1]]
                && grid[v[1][0]][v[1][1]] == grid[v[2][0]][v[2][1]]
                && grid[v[0][0]][v[0][1]] != ""
            {
                return String::from(grid[v[0][0]][v[0][1]]);
            }
        }
        String::from(if moves.len() == 9 { "Draw" } else { "Pending" })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "A",
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![2, 0],
                vec![1, 1],
                vec![2, 1],
                vec![2, 2]
            ])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "B",
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![2, 0]
            ])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "Draw",
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2]
            ])
        );
    }

    #[test]
    fn example_4() {
        assert_eq!("Pending", Solution::tictactoe(vec![vec![0, 0], vec![1, 1]]));
    }
}
