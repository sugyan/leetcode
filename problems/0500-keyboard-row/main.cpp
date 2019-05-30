#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> findWords(vector<string>& words) {
        int d[26];
        for (char c : string("qwertyuiop")) d[c - 'a'] = 1;
        for (char c : string("asdfghjkl"))  d[c - 'a'] = 2;
        for (char c : string("zxcvbnm"))    d[c - 'a'] = 3;
        vector<string> answer;
        for (string word : words) {
            int a = d[tolower(word[0]) - 'a'];
            bool ok = true;
            for (char c : word) {
                if (d[tolower(c) - 'a'] != a) {
                    ok = false;
                    break;
                }
            }
            if (ok) answer.push_back(word);
        }
        return answer;
    }
};

int main() {
    vector<string> words { "Hello", "Alaska", "Dad", "Peace" };
    vector<string> ret = Solution().findWords(words);
    for (string out : ret) {
        cout << out << endl;
    }
}
