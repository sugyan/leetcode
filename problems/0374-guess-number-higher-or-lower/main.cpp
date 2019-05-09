#include <bits/stdc++.h>

using namespace std;

// Forward declaration of guess API.
// @param num, your guess
// @return -1 if my number is lower, 1 if my number is higher, otherwise return 0
int guess(int num) {
    int pick = 6;
    if (num > pick) return -1;
    if (num < pick) return 1;
    return 0;
}

class Solution {
public:
    int guessNumber(int n) {
        int l = 1, r = n, m;
        while (true) {
            m = (r - l) / 2 + l;
            int ret = guess(m);
            if (ret == 0) break;
            if (ret > 0) {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        return m;
    }
};

int main() {
    int ret = Solution().guessNumber(10);
    cout << ret << endl;
}
