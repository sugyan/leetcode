#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int surfaceArea(vector<vector<int>>& grid) {
        int row = grid.size(), col = grid[0].size();
        int answer = 0;
        for (int i = 0; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                if (grid[i][j] > 0) {
                    answer += 2 + 4 * grid[i][j];
                }
                if (i < row - 1) answer -= 2 * min(grid[i][j], grid[i + 1][j]);
                if (j < col - 1) answer -= 2 * min(grid[i][j], grid[i][j + 1]);
            }
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
            { 3, 4 },
        },
        {
            { 1, 0 },
            { 0, 2 },
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
        }
    };
    for (vector<vector<int>>& grid : inputs) {
        int ret = Solution().surfaceArea(grid);
        cout << ret << endl;
    }
}
