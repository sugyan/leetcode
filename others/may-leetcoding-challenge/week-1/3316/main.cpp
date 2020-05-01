#include <bits/stdc++.h>

using namespace std;

bool isBadVersion(int version) {
    return version >= 4;
}

class Solution {
   public:
    int firstBadVersion(int n) {
        int l = 0, r = n;
        while (l < r) {
            int m = l + (r - l) / 2;
            if (isBadVersion(m)) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        return l;
    }
};

int main() {
    int ret = Solution().firstBadVersion(5);
    cout << ret << endl;
}
