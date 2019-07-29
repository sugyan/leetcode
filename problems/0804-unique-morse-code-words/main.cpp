#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int uniqueMorseRepresentations(vector<string>& words) {
        unordered_set<string> us;
        for (string word : words) {
            string s;
            for (char c : word) s += codes[c - 'a'];
            us.insert(s);
        }
        return us.size();
    }
private:
    vector<string> codes {
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."
    };
};

int main() {
    vector<string> words {
        "gin", "zen", "gig", "msg",
    };
    int ret = Solution().uniqueMorseRepresentations(words);
    cout << ret << endl;
}
