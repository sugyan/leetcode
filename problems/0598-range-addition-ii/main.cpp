#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int maxCount(int m, int n, vector<vector<int>>& ops) {
        for (vector<int> op : ops) {
            m = min(m, op[0]);
            n = min(n, op[1]);
        }
        return m * n;
    }
};

int main() {
    int m = 3, n = 3;
    vector<vector<int>> ops {
        { 2, 2 },
        { 3, 3 },
    };
    int ret = Solution().maxCount(m, n, ops);
    cout << ret << endl;
}
