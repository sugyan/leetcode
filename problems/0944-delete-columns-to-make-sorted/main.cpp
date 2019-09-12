#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int minDeletionSize(vector<string>& A) {
        int answer = 0, row = A.size(), col = A[0].size();
        for (int i = 0; i < col; ++i) {
            int ok = true;
            for (int j = 0; j < row - 1; ++j) {
                if (A[j][i] > A[j + 1][i]) {
                    ok = false;
                    break;
                }
            }
            if (!ok) ++answer;
        }
        return answer;
    }
};

int main() {
    vector<vector<string>> inputs {
        {
            "cba",
            "daf",
            "ghi"
        },
        {
            "a",
            "b"
        },
        {
            "zyx",
            "wvu",
            "tsr"
        }
    };
    for (vector<string>& A : inputs) {
        int ret = Solution().minDeletionSize(A);
        cout << ret << endl;
    }
}
