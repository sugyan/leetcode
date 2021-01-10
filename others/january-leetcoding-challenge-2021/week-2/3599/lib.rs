pub struct Solution;

const DIV: usize = 1_000_000_007;

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut v = Vec::new();
        for &instruction in instructions.iter() {
            let i = match v.binary_search(&(instruction - 1)) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            let j = match v.binary_search(&(instruction)) {
                Ok(j) => j + 1,
                Err(j) => j,
            };
            answer = (answer + std::cmp::min(i, v.len() - j)) % DIV;
            if i < v.len() {
                v.insert(i, instruction);
            } else {
                v.push(instruction);
            }
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::create_sorted_array(vec![1, 5, 6, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            4,
            Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2])
        );
    }
}
