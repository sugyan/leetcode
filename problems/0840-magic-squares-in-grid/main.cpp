#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numMagicSquaresInside(vector<vector<int>>& grid) {
        int answer = 0, row = grid.size(), col = grid[0].size();
        if (col < 3 || grid.size() < 3) return answer;
        for (int i = 0; i < row - 2; ++i) {
            for (int j = 0; j < col - 2; ++j) {
                if (grid[1 + i][1 + j] != 5) continue;
                bool ok = true;
                int a[16] { 0 };
                for (int k = 0; k < 9; ++k) {
                    ++a[grid[0 + i + (k / 3)][0 + j + (k % 3)]];
                }
                for (int k = 0; k < 9; ++k) {
                    if (a[k + 1] != 1) {
                        ok = false;
                        break;
                    }
                }
                if (ok &&
                    grid[0 + i][0 + j] + grid[0 + i][1 + j] + grid[0 + i][2 + j] == 15 &&
                    grid[1 + i][0 + j] + grid[1 + i][1 + j] + grid[1 + i][2 + j] == 15 &&
                    grid[2 + i][0 + j] + grid[2 + i][1 + j] + grid[2 + i][2 + j] == 15 &&
                    grid[0 + i][0 + j] + grid[1 + i][0 + j] + grid[2 + i][0 + j] == 15 &&
                    grid[0 + i][1 + j] + grid[1 + i][1 + j] + grid[2 + i][1 + j] == 15 &&
                    grid[0 + i][2 + j] + grid[1 + i][2 + j] + grid[2 + i][2 + j] == 15 &&
                    grid[0 + i][0 + j] + grid[1 + i][1 + j] + grid[2 + i][2 + j] == 15 &&
                    grid[2 + i][0 + j] + grid[1 + i][1 + j] + grid[0 + i][2 + j] == 15) ++answer;
            }
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> grid {
        { 4, 3, 8, 4 },
        { 9, 5, 1, 9 },
        { 2, 7, 6, 2 },
    };
    int ret = Solution().numMagicSquaresInside(grid);
    cout << ret << endl;
}
