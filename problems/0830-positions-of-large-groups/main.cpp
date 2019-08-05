#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> largeGroupPositions(string S) {
        vector<vector<int>> answer;
        int i = 0, j = 0, length = S.length();
        while (i < length) {
            j = i;
            while (j < length && S[j] == S[i]) ++j;
            if (j - i >= 3) {
                answer.push_back({ i, j - 1 });
            }
            i = j;
        }
        return answer;
    }
};

int main() {
    vector<string> inputs {
        "abbxxxxzzy",
        "abc",
        "abcdddeeeeaabbbcd",
    };
    for (string& S : inputs) {
        vector<vector<int>> ret = Solution().largeGroupPositions(S);
        for (vector<int> out : ret) {
            cout << '[' << out[0] << ", " << out[1] << ']' << " ";
        }
        cout << endl;
    }
}
