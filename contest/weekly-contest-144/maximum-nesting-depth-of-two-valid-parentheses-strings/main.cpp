#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> maxDepthAfterSplit(string seq) {
        vector<int> answer;
        int nested = 0;
        for (int i = 0, length = seq.length(); i < length; ++i) {
            if (seq[i] == '(') ++nested;
            if (seq[i] == ')') --nested;
            if (nested > 1 || nested == 1 && seq[i] == ')') {
                answer.push_back(1);
            } else {
                answer.push_back(0);
            }
        }
        return answer;
    }
};

int main() {
    vector<string> inputs {
        "(()())",
        "()(())()",
    };
    for (string seq : inputs) {
        vector<int> ret = Solution().maxDepthAfterSplit(seq);
        copy(ret.begin(), ret.end(), ostream_iterator<int>(cout, ","));
        cout << endl;
    }
}
