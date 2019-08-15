#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<vector<int>> transpose(vector<vector<int>>& A) {
        int row = A.size(), col = A[0].size();
        vector<vector<int>> B(col, vector<int>(row));
        for (int i = 0; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                B[j][i] = A[i][j];
            }
        }
        return B;
    }
};

int main() {
    vector<vector<vector<int>>> inputs {
        {
            { 1, 2, 3 },
            { 4, 5, 6 },
            { 7, 8, 9 }
        },
        {
            { 1, 2, 3 },
            { 4, 5, 6 },
        }
    };
    for (vector<vector<int>>& A : inputs) {
        vector<vector<int>> ret = Solution().transpose(A);
        for (vector<int>& out : ret) {
            copy(out.begin(), out.end(), experimental::make_ostream_joiner(cout, ", "));
            cout << endl;
        }
        cout << endl;
    }
}
