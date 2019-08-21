#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<string> uncommonFromSentences(string A, string B) {
        unordered_map<string, int> um;
        for (int i = 0;;) {
            int next = A.find_first_of(' ', i + 1);
            ++um[A.substr(i, next - i)];
            if (next == string::npos) break;
            i = next + 1;
        }
        for (int i = 0;;) {
            int next = B.find_first_of(' ', i + 1);
            ++um[B.substr(i, next - i)];
            if (next == string::npos) break;
            i = next + 1;
        }
        vector<string> answer;
        for (const pair<string, int>& p : um) {
            if (p.second == 1) answer.push_back(p.first);
        }
        return answer;
    }
};

int main() {
    vector<vector<string>> inputs {
        {
            "this apple is sweet",
            "this apple is sour",
        },
        {
            "apple apple",
            "banana",
        },
    };
    for (vector<string>& input : inputs) {
        vector<string> ret = Solution().uncommonFromSentences(input[0], input[1]);
        copy(ret.begin(), ret.end(), experimental::make_ostream_joiner(cout, ", "));
        cout << endl;
    }
}
