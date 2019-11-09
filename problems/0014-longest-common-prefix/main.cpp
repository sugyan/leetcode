#include <bits/stdc++.h>

using namespace std;

class Solution {
   public:
    string longestCommonPrefix(vector<string>& strs) {
        if (strs.empty()) return "";
        string answer;
        for (int i = 0; i < strs[0].length(); ++i) {
            for (int j = 1; j < strs.size(); ++j) {
                if (strs[j][i] != strs[0][i]) return answer;
            }
            answer += strs[0][i];
        }
        return answer;
    }
};

int main() {
    vector<vector<string>> inputs{
        {"flower", "flow", "flight"},
        {"dog", "racecar", "car"},
    };

    for (vector<string>& strs : inputs) {
        string ret = Solution().longestCommonPrefix(strs);
        cout << ret << endl;
    }
}
