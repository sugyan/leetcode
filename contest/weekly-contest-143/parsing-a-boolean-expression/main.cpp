#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool parseBoolExpr(string expression) {
        vector<bool> ret = parse(expression);
        return ret.front();
    }
private:
    vector<bool> parse(string s) {
        vector<bool> answer;
        for (int i = 0, length = s.length(); i < length; ++i) {
            if (s[i] == '!' || s[i] == '&' || s[i] == '|') {
                char c = s[i];
                int paren = 1; i += 2;
                int start = i;
                while (paren > 0) {
                    if (s[i] == '(') ++paren;
                    if (s[i] == ')') --paren;
                    ++i;
                }
                vector<bool> values = parse(s.substr(start, i - start - 1));
                if (c == '!') {
                    answer.push_back(!values[0]);
                }
                if (c == '&') {
                    bool ret = true;
                    for (bool b : values) ret &= b;
                    answer.push_back(ret);
                }
                if (c == '|') {
                    bool ret = false;
                    for (bool b : values) ret |= b;
                    answer.push_back(ret);
                }
            }
            if (s[i] == 'f' || s[i] == 't') {
                if (s[i] == 'f') answer.push_back(false);
                if (s[i] == 't') answer.push_back(true);
                ++i;
            }
        }
        return answer;
    }
};

int main() {
    vector<string> inputs {
        "!(f)",
        "|(f,t)",
        "&(t,f)",
        "|(&(t,f,t),!(t))",
    };
    for (string expression : inputs) {
        bool ret = Solution().parseBoolExpr(expression);
        cout << ret << endl;
    }
}