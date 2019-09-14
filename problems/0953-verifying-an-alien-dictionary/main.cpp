#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool isAlienSorted(vector<string>& words, string order) {
        int index[26];
        for (int i = 0; i < 26; ++i) index[order[i] - 'a'] = i;
        for (int i = 0, size = words.size(); i < size - 1; ++i) {
            int l = min(words[i].length(), words[i + 1].length());
            bool ok = false;
            for (int j = 0; j < l; ++j) {
                if (index[words[i][j] - 'a'] > index[words[i + 1][j] - 'a']) {
                    return false;
                }
                if (index[words[i][j] - 'a'] < index[words[i + 1][j] - 'a']) {
                    ok = true;
                    break;
                }
            }
            if (!ok && words[i].length() > words[i + 1].length()) return false;
        }
        return true;
    }
};

int main() {
    vector<pair<vector<string>, string>> inputs {
        {
            {
                "hello",
                "leetcode"
            },
            "hlabcdefgijkmnopqrstuvwxyz"
        },
        {
            {
                "word",
                "world",
                "row"
            },
            "worldabcefghijkmnpqstuvxyz"
        },
        {
            {
                "apple",
                "app"
            },
            "abcdefghijklmnopqrstuvwxyz"
        }
    };
    for (pair<vector<string>, string>& input : inputs) {
        bool ret = Solution().isAlienSorted(input.first, input.second);
        cout << ret << endl;
    }
}
