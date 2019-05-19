#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string removeDuplicates(string S) {
        while (true) {
            bool flg = true;
            for (int i = 1, length = S.length(); i < length; ++i) {
                if (S[i] == S[i - 1]) {
                    S.erase(S.begin() + i);
                    S.erase(S.begin() + i - 1);
                    flg = false;
                    break;
                }
            }
            if (flg) break;
        }
        return S;
    }
};

int main() {
    string S = "abbaca";
    string ret = Solution().removeDuplicates(S);
    cout << ret << endl;
}
