#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> numSmallerByFrequency(vector<string>& queries, vector<string>& words) {
        vector<int> w(words.size());
        for (int i = 0; i < w.size(); ++i) {
            map<char, int> m;
            for (char& c : words[i]) ++m[c];
            w[i] = m.begin()->second;
        }
        // copy(w.begin(), w.end(), ostream_iterator<int>(cout, " - "));
        // cout << endl;
        sort(w.begin(), w.end());
        vector<int> answer(queries.size());
        for (int i = 0; i < answer.size(); ++i) {
            map<char, int> m;
            for (char& c : queries[i]) ++m[c];
            // cout << m.begin()->second << endl;
            answer[i] = w.end() - upper_bound(w.begin(), w.end(), m.begin()->second);
        }
        return answer;
    }
};

int main() {
    vector<pair<vector<string>, vector<string>>> inputs {
        {
            { "cbd" },
            { "zaaaz" },
        },
        {
            { "bbb", "cc" },
            { "a", "aa", "aaa", "aaaa" },
        },
        {
            {"bba","abaaaaaa","aaaaaa","bbabbabaab","aba","aa","baab","bbbbbb","aab","bbabbaabb"},
            {"aaabbb","aab","babbab","babbbb","b","bbbbbbbbab","a","bbbbbbbbbb","baaabbaab","aa"},
        }
    };
    for (auto& input : inputs) {
        vector<int> ret = Solution().numSmallerByFrequency(input.first, input.second);
        copy(ret.begin(), ret.end(), ostream_iterator<int>(cout, ", "));
        cout << endl;
    }
}
