pub struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut d: [i32; 26] = [0; 26];
        for &task in tasks.iter() {
            d[(task as u8 - b'A') as usize] += 1;
        }
        let (mut val, mut num) = (0, 0);
        for &v in d.iter() {
            match v.cmp(&val) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => num += 1,
                std::cmp::Ordering::Greater => {
                    val = v;
                    num = 1;
                }
            }
        }
        std::cmp::max(tasks.len() as i32, (val - 1) * (n + 1) + num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            8,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            6,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            16,
            Solution::least_interval(
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                2
            )
        );
    }
}
