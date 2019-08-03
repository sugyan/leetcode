#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<int> shortestToChar(string S, char C) {
        int d, length = S.length();
        vector<int> answer(length, length);
        for (int i = S.find_first_of(C); i < length; ++i) {
            if (S[i] == C) d = 0;
            answer[i] = min(answer[i], d++);
        }
        for (int i = S.find_last_of(C); i >= 0; --i) {
            if (S[i] == C) d = 0;
            answer[i] = min(answer[i], d++);
        }
        return answer;
    }
};

int main() {
    string S = "loveleetcode";
    char C = 'e';
    vector<int> ret = Solution().shortestToChar(S, C);
    copy(ret.begin(), ret.end(), experimental::make_ostream_joiner(cout, ", "));
    cout << endl;
}
