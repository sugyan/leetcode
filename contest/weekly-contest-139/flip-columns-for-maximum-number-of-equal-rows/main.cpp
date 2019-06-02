#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int maxEqualRowsAfterFlips(vector<vector<int>>& matrix) {
        int row = matrix.size(), col = matrix[0].size();
        int answer = 0;
        for (int i = 0; i < row - 1; ++i) {
            int count = 0;
            for (int j = i + 1; j < row; ++j) {
                bool flg = true;
                for (int k = 0; k < col; ++k) {
                    if (matrix[i][k] != matrix[j][k]) {
                        flg = false;
                        break;
                    }
                }
                if (!flg) {
                    flg = true;
                    for (int k = 0; k < col; ++k) {
                        if (matrix[i][k] != 1 - matrix[j][k]) {
                            flg = false;
                            break;
                        }
                    }
                }
                if (flg) ++count;
            }
            answer = max(answer, count);
        }
        return answer + 1;
    }
};

int main() {
    vector<vector<int>> matrix {
        { 0, 0, 0 },
        { 0, 0, 1 },
        { 1, 1, 0 },
    };
    int ret = Solution().maxEqualRowsAfterFlips(matrix);
    cout << ret << endl;
}
