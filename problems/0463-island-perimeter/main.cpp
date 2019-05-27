#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int islandPerimeter(vector<vector<int>>& grid) {
        int answer = 0;
        for (int i = 0, row = grid.size(); i < row; ++i) {
            for (int j = 0, col = grid[i].size(); j < col; ++j) {
                if (grid[i][j] == 1) {
                    if (i == 0 || grid[i - 1][j] == 0) ++answer;
                    if (i == row - 1 || grid[i + 1][j] == 0) ++answer;
                    if (j == 0 || grid[i][j - 1] == 0) ++answer;
                    if (j == col - 1 || grid[i][j + 1] == 0) ++answer;
                }
            }
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> grid {
        { 0, 1, 0, 0 },
        { 1, 1, 1, 0 },
        { 0, 1, 0, 0 },
        { 1, 1, 0, 0 },
    };
    int ret = Solution().islandPerimeter(grid);
    cout << ret << endl;
}
