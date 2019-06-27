#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<vector<int>> imageSmoother(vector<vector<int>>& M) {
        int row = M.size(), col = M[0].size();
        vector<vector<int>> N(row, vector<int>(col));
        for (int i = 0; i < row; ++i) {
            int sum = M[i][0];
            for (int j = 0; j < col; ++j) {
                if (j < col - 1) sum += M[i][j + 1];
                if (j > 1) sum -= M[i][j - 2];
                N[i][j] = sum;
            }
        }
        for (int j = 0; j < col; ++j) {
            int sum = N[0][j];
            for (int i = 0; i < row; ++i) {
                if (i < row - 1) sum += N[i + 1][j];
                if (i > 1) sum -= N[i - 2][j];
                int div = 9;
                if (i == 0 || i == row - 1) div = div * 2 / 3;
                if (j == 0 || j == col - 1) div = div * 2 / 3;
                if (row == 1) div /= 2;
                if (col == 1) div /= 2;
                M[i][j] = int(1.0 * sum / div);
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
