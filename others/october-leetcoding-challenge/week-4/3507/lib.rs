pub struct Solution {}

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut v = vec![false; n as usize + 1];
        for i in 0..=n as usize {
            if v[i] {
                continue;
            }
            for j in 1.. {
                if i + j * j > n as usize {
                    break;
                }
                v[i + j * j] = true;
            }
        }
        v[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::winner_square_game(1));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::winner_square_game(2));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::winner_square_game(4));
    }

    #[test]
    fn example_4() {
        assert_eq!(false, Solution::winner_square_game(7));
    }

    #[test]
    fn example_5() {
        assert_eq!(false, Solution::winner_square_game(17));
    }
}
