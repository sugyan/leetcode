#include <bits/stdc++.h>

using namespace std;

class Solution {
   public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        if (matrix.empty() || matrix[0].empty()) return false;
        vector<int> l, r;
        for (int i = 0; i < matrix.size(); ++i) {
            l.push_back(matrix[i].front());
            r.push_back(matrix[i].back());
        }
        int ll = distance(l.begin(), upper_bound(l.begin(), l.end(), target));
        int rr = distance(r.begin(), lower_bound(r.begin(), r.end(), target));
        for (int i = 0; i < ll; ++i) {
            if (i < rr) continue;
            auto lb = lower_bound(matrix[i].begin(), matrix[i].end(), target);
            if (*lb == target) {
                return true;
            }
        }
        return false;
    }
};

int main() {
    vector<vector<int>> matrix = {
        {1, 4, 7, 11, 15},    {2, 5, 8, 12, 19},    {3, 6, 9, 16, 22},
        {10, 13, 14, 17, 24}, {18, 21, 23, 26, 30},
    };
    cout << Solution().searchMatrix(matrix, 5) << endl;
    cout << Solution().searchMatrix(matrix, 20) << endl;
}
