pub struct Solution {}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut v: Vec<(i32, &Vec<i32>)> = points
            .iter()
            .map(|e| (e[0] * e[0] + e[1] * e[1], e))
            .collect();
        let (mut lo, mut hi) = (0, points.len() - 1);
        loop {
            let p = Solution::helper(&mut v, lo, hi);
            if p == k as usize - 1 {
                break;
            }
            if p < k as usize - 1 {
                lo = p + 1;
            } else {
                hi = p - 1;
            }
        }
        v.iter().take(k as usize).map(|e| e.1.clone()).collect()
    }

    fn helper(v: &mut Vec<(i32, &Vec<i32>)>, lo: usize, hi: usize) -> usize {
        let (mut l, mut r) = (lo + 1, hi);
        loop {
            while l <= r && v[l].0 < v[lo].0 {
                l += 1;
            }
            while l <= r && v[r].0 > v[lo].0 {
                r -= 1;
            }
            if l >= r {
                break;
            }
            v.swap(l, r);
            l += 1;
            r -= 1;
        }
        v.swap(lo, r);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret: Vec<Vec<i32>> = Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1);
        ret.sort();
        assert_eq!(vec![vec![-2, 2]], ret);
    }

    #[test]
    fn example_2() {
        let mut ret: Vec<Vec<i32>> =
            Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
        ret.sort();
        assert_eq!(vec![vec![-2, 4], vec![3, 3]], ret);
    }
}
