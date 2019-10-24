#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int countCharacters(vector<string>& words, string chars) {
        int d[26] { 0 };
        for (char& c : chars) ++d[c - 'a'];
        int answer = 0;
        for (string& word : words) {
            int dd[26] { 0 };
            bool ok = true;
            for (char& c : word) {
                if (++dd[c - 'a'] > d[c - 'a']) {
                    ok = false;
                    break;
                }
            }
            if (ok) answer += word.length();
        }
        return answer;
    }
};

int main() {
    vector<pair<vector<string>, string>> inputs {
        {
            { "cat", "bt", "hat", "tree" },
            "atach",
        },
        {
            { "hello", "world", "leetcode" },
            "welldonehoneyr",
        },
    };
    for (auto& input : inputs) {
        int ret = Solution().countCharacters(input.first, input.second);
        cout << ret << endl;
    }
}
