#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> flipAndInvertImage(vector<vector<int>>& A) {
        for (vector<int>& row : A) {
            reverse(row.begin(), row.end());
            for (int& col : row) {
                col = 1 - col;
            }
        }
        return A;
    }
};

int main() {
    vector<vector<vector<int>>> inputs {
        {
            { 1, 1, 0 },
            { 1, 0, 1 },
            { 0, 0, 0 },
        },
        {
            { 1, 1, 0, 0 },
            { 1, 0, 0, 1 },
            { 0, 1, 1, 1 },
            { 1, 0, 1, 0 },
        },
    };
    for (vector<vector<int>>& A : inputs) {
        vector<vector<int>> ret = Solution().flipAndInvertImage(A);
        for (vector<int> out : ret) {
            copy(out.begin(), out.end(), ostream_iterator<int>(cout, ", "));
            cout << endl;
        }
        cout << endl;
    }
}
