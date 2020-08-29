pub struct Solution {}

impl Solution {
    pub fn pancake_sort(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut answer: Vec<i32> = Vec::new();
        for i in (0..a.len()).rev() {
            let (mut maxval, mut maxidx) = (std::i32::MIN, 0);
            #[allow(clippy::needless_range_loop)]
            for j in 0..=i {
                if a[j] > maxval {
                    maxval = a[j];
                    maxidx = j;
                }
            }
            if maxidx != i {
                if maxidx > 0 {
                    answer.push(maxidx as i32 + 1);
                    Solution::flip(&mut a, maxidx + 1);
                }
                answer.push(i as i32 + 1);
                Solution::flip(&mut a, i + 1);
            }
        }
        answer
    }
    fn flip(a: &mut [i32], k: usize) {
        for i in 0..k / 2 {
            a.swap(i, k - 1 - i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut v = vec![1, 3, 4, 2];
        let ret = Solution::pancake_sort(v.clone());
        assert!(ret.len() < v.len() * 10);
        for &k in ret.iter() {
            for i in 0..k as usize / 2 {
                v.swap(i, k as usize - 1 - i);
            }
        }
        assert_eq!(vec![1, 2, 3, 4], v);
    }

    #[test]
    fn example_2() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(v, Solution::pancake_sort(vec![1, 2, 3]));
    }
}
