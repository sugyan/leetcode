pub struct Solution;

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut lo, mut hi) = (1, n);
        while lo < hi {
            let m = lo + (hi - lo) / 2;
            match guess(m) {
                -1 => hi = m - 1,
                1 => lo = m + 1,
                _ => return m,
            }
        }
        lo
    }
}
