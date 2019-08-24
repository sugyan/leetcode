#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numSpecialEquivGroups(vector<string>& A) {
        set<pair<string, string>> s;
        int l = A[0].length();
        for (string a : A) {
            string even, odd;
            for (int i = 0; i < l; ++i) {
                if (i % 2 == 0) even += a[i];
                else odd += a[i];
            }
            sort(even.begin(), even.end());
            sort(odd.begin(), odd.end());
            s.insert({ even, odd });
            // s.insert(even + "-" + odd);
        }
        return s.size();
    }
};

int main() {
    vector<vector<string>> inputs {
        { "a", "b", "c", "a", "c", "c" },
        { "aa", "bb", "ab", "ba" },
        { "abc", "acb", "bac", "bca", "cab", "cba" },
        { "abcd", "cdab", "adcb", "cbad" },
    };
    for (vector<string>& A : inputs) {
        int ret = Solution().numSpecialEquivGroups(A);
        cout << ret << endl;
    }
}
