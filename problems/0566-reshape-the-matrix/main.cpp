#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> matrixReshape(vector<vector<int>>& nums, int r, int c) {
        int rr = nums.size(), cc = nums[0].size();
        if (rr * cc != r * c) return nums;
        vector<vector<int>> answer(r, vector<int>(c));
        for (int i = 0; i < r * c; ++i) {
            answer[i / c][i % c] = nums[i / cc][i % cc];
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> nums {
        { 1, 2 },
        { 3, 4 },
    };
    vector<pair<int, int>> rc { { 1, 4 }, { 2, 4 } };
    for (pair<int, int> p : rc) {
        vector<vector<int>> ret = Solution().matrixReshape(nums, p.first, p.second);
        for (vector<int> row : ret) {
            for (int col : row) cout << col << " ";
            cout << endl;
        }
    }
}
