#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    void gen(vector<string> &v, string str, int l, int r) {
        if (l == 0 && r == 0) v.push_back(str);
        if (l > 0) gen(v, str + "(", l - 1, r);
        if (r > l) gen(v, str + ")", l, r - 1);
    }
    vector<string> generateParenthesis(int n) {
        vector<string> answer;
        gen(answer, "", n, n);
        return answer;
    }
};

int main() {
    int n = 3;
    vector<string> ret = Solution().generateParenthesis(n);
    for (string out : ret) {
        cout << out << endl;
    }
}
