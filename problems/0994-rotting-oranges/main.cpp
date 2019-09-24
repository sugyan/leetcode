#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int orangesRotting(vector<vector<int>>& grid) {
        int row = grid.size(), col = grid[0].size();
        vector<pair<int, int>> v;
        for (int i = 0; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                if (grid[i][j] == 2) {
                    v.push_back({ i, j });
                }
            }
        }
        bfs(grid, v, row, col, 2);
        int answer = 2;
        for (vector<int>& r : grid) {
            for (int& c : r) {
                if (c == 1) return -1;
                answer = max(answer, c);
            }
        }
        return answer - 2;
    }
private:
    void bfs(vector<vector<int>>& grid, vector<pair<int, int>>& target, int& row, int& col, int n) {
        vector<pair<int, int>> v;
        for (pair<int, int>& p : target) {
            if (grid[p.first][p.second] == 1) grid[p.first][p.second] = n;
            if (p.first > 0 && grid[p.first - 1][p.second] == 1) v.push_back({ p.first - 1, p.second });
            if (p.second > 0 && grid[p.first][p.second - 1] == 1) v.push_back({ p.first, p.second - 1 });
            if (p.first < row - 1 && grid[p.first + 1][p.second] == 1) v.push_back({ p.first + 1, p.second });
            if (p.second < col - 1 && grid[p.first][p.second + 1] == 1) v.push_back({ p.first, p.second + 1 });
        }
        if (!v.empty()) bfs(grid, v, row, col, n + 1);
    }
};

int main() {
    vector<vector<vector<int>>> inputs {
        {
            { 2, 1, 1 },
            { 1, 1, 0 },
            { 0, 1, 1 },
        },
        {
            { 2, 1, 1 },
            { 0, 1, 1 },
            { 1, 0, 1 },
        },
        {
            { 0, 2 },
        },
    };
    for (vector<vector<int>>& grid : inputs) {
        int ret = Solution().orangesRotting(grid);
        cout << ret << endl;
    }
}
