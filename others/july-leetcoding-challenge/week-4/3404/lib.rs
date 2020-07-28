pub struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut d: [i32; 26] = [0; 26];
        let (mut val, mut num) = (0, 0);
        for &task in tasks.iter() {
            let i = (task as u8 - b'A') as usize;
            d[i] += 1;
            if d[i] >= val {
                if d[i] > val {
                    val = d[i];
                    num = 0;
                }
                num += 1;
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
