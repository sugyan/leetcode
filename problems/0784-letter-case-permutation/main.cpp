#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<string> letterCasePermutation(string S) {
        vector<string> answer;
        permute(S, answer, 0);
        return answer;
    }
private:
    void permute(string s, vector<string>& answer, int idx) {
        if (idx == s.length()) {
            answer.push_back(s);
            return;
        }
        permute(s, answer, idx + 1);
        if (isalpha(s[idx])) {
            s[idx] ^= (1 << 5);
            permute(s, answer, idx + 1);
            s[idx] ^= (1 << 5);
        }
    }
};

int main() {
    vector<string> inputs {
        "a1b2",
        "3z4",
        "12345",
    };
    for (string S : inputs) {
        vector<string> ret = Solution().letterCasePermutation(S);
        copy(ret.begin(), ret.end(), experimental::make_ostream_joiner(cout, ", "));
        cout << endl;
    }
}
