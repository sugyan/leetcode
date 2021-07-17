pub struct Solution;

impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let v = arr
            .iter()
            .enumerate()
            .filter_map(|(i, &a)| if a == 1 { Some(i) } else { None })
            .collect::<Vec<_>>();
        if arr.len() < 3 || v.len() % 3 != 0 {
            return [-1, -1].to_vec();
        }
        if v.is_empty() {
            return [0, 2].to_vec();
        }
        let chunks = v.chunks(v.len() / 3).collect::<Vec<_>>();
        let i = chunks[0][v.len() / 3 - 1] + arr.len() - v[v.len() - 1] - 1;
        let j = chunks[1][v.len() / 3 - 1] + arr.len() - v[v.len() - 1];
        if arr.len() - chunks[2][0] > j - i - 1 || arr.len() - chunks[2][0] > i + 1 {
            return [-1, -1].to_vec();
        }
        for k in 0..(i + 1).min(j - i - 1).min(arr.len() - j) {
            if arr[i - k] != arr[j - 1 - k] || arr[i - k] != arr[arr.len() - 1 - k] {
                return [-1, -1].to_vec();
            }
        }
        [i as i32, j as i32].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![0, 3], Solution::three_equal_parts(vec![1, 0, 1, 0, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![-1, -1],
            Solution::three_equal_parts(vec![1, 1, 0, 1, 1])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![0, 2], Solution::three_equal_parts(vec![1, 1, 0, 0, 1]));
    }
}
