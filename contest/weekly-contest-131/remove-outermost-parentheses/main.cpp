#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string removeOuterParentheses(string S) {
        int depth = 0;
        string answer = "";
        for (int i = 0, l = S.length(); i < l; ++i) {
            if (S[i] == '(') {
                ++depth;
                if (depth > 1) {
                    answer += S[i];
                }
            } else {
                --depth;
                if (depth >= 1) {
                    answer += S[i];
                }
            }
        }
        return answer;
    }
};
