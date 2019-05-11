#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    char findTheDifference(string s, string t) {
        char answer = 0;
        for (char c : s) answer ^= c;
        for (char c : t) answer ^= c;
        return answer;
    }
};

int main() {
    string s = "abcd";
    string t = "abcde";
    char ret = Solution().findTheDifference(s, t);
    cout << ret << endl;
}
