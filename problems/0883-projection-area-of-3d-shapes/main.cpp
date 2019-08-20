#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int projectionArea(vector<vector<int>>& grid) {
        int row = grid.size(), col = grid[0].size();
        int answer = 0;
        for (int i = 0; i < row; ++i) {
            int xz = 0, yz = 0;
            for (int j = 0; j < col; ++j) {
                if (grid[i][j] > 0) ++answer;
                xz = max(xz, grid[i][j]);
                yz = max(yz, grid[j][i]);
            }
            answer += xz + yz;
        }
        return answer;
    }
};

int main() {
    vector<vector<vector<int>>> inputs {
        {
            { 2 },
        },
        {
            { 1, 2 },
            { 3, 4 }
        },
        {
            { 1, 0 },
            { 0, 2 }
        },
        {
            { 1, 1, 1 },
            { 1, 0, 1 },
            { 1, 1, 1 },
        },
        {
            { 2, 2, 2 },
            { 2, 1, 2 },
            { 2, 2, 2 },
        },
    };
    for (vector<vector<int>>& grid : inputs) {
        int ret = Solution().projectionArea(grid);
        cout << ret << endl;
    }
}
