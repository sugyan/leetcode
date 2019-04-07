#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<bool> camelMatch(vector<string>& queries, string pattern) {
        vector<bool> answer;
        for (int i = 0, l = queries.size(); i < l; ++i) {
            int m = 0, n = 0;
            while (m < queries[i].length() && n < pattern.length()) {
                if (queries[i][m] == pattern[n]) {
                    ++n;
                } else {
                    if (pattern[n] < 'a' && queries[i][m] < 'a') {
                        break;
                    }
                }
                ++m;
            }
            if (n < pattern.length()) {
                answer.push_back(false);
                continue;
            }
            bool ok = true;
            for (int j = m, ll = queries[i].length(); j < ll; ++j) {
                if (queries[i][j] < 'a') {
                    ok = false;
                    break;
                }
            }
            answer.push_back(ok);
        }
        return answer;
    }
};
