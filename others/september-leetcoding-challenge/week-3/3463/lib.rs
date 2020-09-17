pub struct Solution {}

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut d = (0, 1);
        let mut p = (0, 0);
        for c in instructions.chars() {
            match c {
                'G' => p = (p.0 + d.0, p.1 + d.1),
                'L' => d = (-d.1, d.0),
                'R' => d = (d.1, -d.0),
                _ => {}
            }
        }
        p == (0, 0) || d != (0, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::is_robot_bounded(String::from("GGLLGG")));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::is_robot_bounded(String::from("GG")));
    }

    #[test]
    fn example_3() {
        assert_eq!(true, Solution::is_robot_bounded(String::from("GL")));
    }
}
