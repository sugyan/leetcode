#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool isToeplitzMatrix(vector<vector<int>>& matrix) {
        int row = matrix.size(), col = matrix[0].size();
        for (int i = 1; i < row; ++i) {
            for (int j = 1; j < col; ++j) {
                if (matrix[i][j] != matrix[i - 1][j - 1]) return false;
            }
        }
        return true;
    }
};

int main() {
    vector<vector<vector<int>>> inputs {
        {
            { 1, 2, 3, 4 },
            { 5, 1, 2, 3 },
            { 9, 5, 1, 2 },
        },
        {
            {1, 2},
            {2, 2},
        },
    };
    for (vector<vector<int>> matrix : inputs) {
        bool ret = Solution().isToeplitzMatrix(matrix);
        cout << ret << endl;
        // break;
    }
}
