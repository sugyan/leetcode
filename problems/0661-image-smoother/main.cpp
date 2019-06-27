#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<vector<int>> imageSmoother(vector<vector<int>>& M) {
        int row = M.size(), col = M[0].size();
        for (int i = 0; i < row; ++i) {
            int prev = 0;
            for (int j = 0; j < col - 1; ++j) {
                int tmp = M[i][j];
                M[i][j] += prev + M[i][j + 1];
                prev = tmp;
            }
            M[i][col - 1] += prev;
        }
        for (int j = 0; j < col; ++j) {
            int prev = 0;
            for (int i = 0; i < row - 1; ++i) {
                int tmp = M[i][j];
                M[i][j] += prev + M[i + 1][j];
                prev = tmp;
            }
            M[row - 1][j] += prev;
        }
        for (int i = 0; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                int div = 9;
                if (i == 0 || i == row - 1) div = div * 2 / 3;
                if (j == 0 || j == col - 1) div = div * 2 / 3;
                if (row == 1) div /= 2;
                if (col == 1) div /= 2;
                M[i][j] = int(1.0 * M[i][j] / div);
            }
        }
        return M;
    }
};

int main() {
    vector<vector<int>> M {
        { 1, 1, 1 },
        { 1, 0, 1 },
        { 1, 1, 1 },
    };
    vector<vector<int>> ret = Solution().imageSmoother(M);
    for (vector<int> row : ret) {
        copy(row.begin(), row.end(), experimental::make_ostream_joiner(cout, ", "));
        cout << endl;
    }
}
