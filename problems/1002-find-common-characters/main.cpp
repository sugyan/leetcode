#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> commonChars(vector<string>& A) {
        int a[26], b[26];
        for (int i = 0; i < 26; ++i) a[i] = 100;
        for (string s : A) {
            for (int i = 0; i < 26; ++i) b[i] = 0;
            for (char c : s) ++b[c - 'a'];
            for (int i = 0; i < 26; ++i) a[i] = min(a[i], b[i]);
        }
        vector<string> answer;
        for (int i = 0; i < 26; ++i) {
            for (int j = 0; j < a[i]; ++j) answer.push_back(string(1, 'a' + i));
        }
        return answer;
    }
};

int main() {
    vector<vector<string>> inputs {
        { "bella", "label", "roller" },
        { "cool", "lock", "cook" },
    };
    for (vector<string>& A : inputs) {
        vector<string> ret = Solution().commonChars(A);
        copy(ret.begin(), ret.end(), ostream_iterator<string>(cout, ", "));
        cout << endl;
    }
}
