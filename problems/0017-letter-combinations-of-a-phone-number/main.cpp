#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> letterCombinations(string digits) {
        if (digits.empty()) return vector<string>();
        vector<string> v { "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz" };
        deque<string> q { "" };
        for (int i = 0, length = digits.length(); i < length; ++i) {
            while (q.front().length() == i) {
                string s = q.front();
                q.pop_front();
                for (char c : v[digits[i] - '2']) {
                    q.push_back(s + c);
                }
            }
        }
        return vector<string>(q.begin(), q.end());
    }
};
