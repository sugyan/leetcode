#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<bool> prefixesDivBy5(vector<int>& A) {
        vector<bool> answer(A.size(), false);
        int n = 0;
        for (int i = 0, size = A.size(); i < size; ++i) {
            n = n * 2 + A[i];
            answer[i] = (n %= 5) == 0;
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> inputs {
        { 0, 1, 1 },
        { 1, 1, 1 },
        { 0, 1, 1, 1, 1, 1 },
        { 1, 1, 1, 0, 1 },
    };
    for (vector<int>& A : inputs) {
        vector<bool> ret = Solution().prefixesDivBy5(A);
        copy(ret.begin(), ret.end(), ostream_iterator<bool>(cout, ", "));
        cout << endl;
    }
}
