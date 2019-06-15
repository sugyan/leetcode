#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> permute(string S) {
        vector<vector<char>> vv;
        for (int i = 0, length = S.length(); i < length; ++i) {
            if (S[i] == '{') {
                ++i;
                vector<char> v;
                while (S[i] != '}') {
                    if (S[i] != ',') v.push_back(S[i]);
                    ++i;
                }
                vv.push_back(v);
            } else {
                vv.push_back({ S[i] });
            }
        }
        vector<string> answer;
        generate(vv, "", answer);
        sort(answer.begin(), answer.end());
        return answer;
    }
private:
    void generate(vector<vector<char>> vv, string s, vector<string>& answer) {
        if (s.length() == vv.size()) {
            answer.push_back(s);
            return;
        }
        vector<char> v = vv[s.length()];
        for (char c : v) {
            generate(vv, s + c, answer);
        }
    }
};

int main() {
    vector<string> inputs {
        "{a,b}c{d,e}f",
        "abcd",
        "{a,b}{z,x,y}",
    };
    for (string S : inputs) {
        vector<string> ret = Solution().permute(S);
        for (string out : ret) cout << out << " ";
        cout << endl;
    }
}
