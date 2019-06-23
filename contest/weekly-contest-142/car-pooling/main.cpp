#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool carPooling(vector<vector<int>>& trips, int capacity) {
        vector<int> v(1001);
        for (vector<int> trip : trips) {
            for (int i = trip[1]; i < trip[2]; ++i) {
                v[i] += trip[0];
            }
        }
        for (int i : v) {
            if (i > capacity) return false;
        }
        return true;
    }
};

int main() {
    vector<pair<vector<vector<int>>, int>> inputs {
        { { { 2, 1, 5 }, { 3, 3, 7 } }, 4 },
        { { { 2, 1, 5 }, { 3, 3, 7 } }, 5 },
        { { { 2, 1, 5 }, { 3, 5, 7 } }, 3 },
        { { { 3, 2, 7 }, { 3, 7, 9 }, { 8, 3, 9 } }, 11 },
    };
    for (pair<vector<vector<int>>, int> input : inputs) {
        bool ret = Solution().carPooling(input.first, input.second);
        cout << ret << endl;
    }
}
