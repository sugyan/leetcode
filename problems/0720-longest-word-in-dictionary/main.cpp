#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string longestWord(vector<string>& words) {
        unordered_set<string> us;
        sort(words.begin(), words.end());
        string answer;
        for (string word : words) {
            if (word.length() == 1 || us.find(word.substr(0, word.length() - 1)) != us.end()) {
                if (word.length() > answer.length()) answer = word;
                us.insert(word);
            }
        }
        return answer;
    }
};

int main() {
    vector<vector<string>> inputs {
        { "w", "wo", "wor", "worl", "world" },
        { "a", "banana", "app", "appl", "ap", "apply", "apple" },
    };
    for (vector<string> words : inputs) {
        string ret = Solution().longestWord(words);
        cout << ret << endl;
    }
}
