#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string shortestCompletingWord(string licensePlate, vector<string>& words) {
        map<int, vector<string>> m;
        for (string word : words) {
            m[word.length()].push_back(word);
        }
        int d[26] { 0 };
        int l = 0;
        for (char c : licensePlate) {
            if (isalpha(c)) {
                ++l;
                ++d[tolower(c) - 'a'];
            }
        }
        for (pair<int, vector<string>> p : m) {
            if (p.first < l) continue;
            for (string word : p.second) {
                if (isCompleting(d, word)) return word;
            }
        }
        return "";
    }
private:
    bool isCompleting(int target[26], string& word) {
        int d[26] { 0 };
        for (char c : word) {
            if (target[c - 'a'] > 0) ++d[c - 'a'];
        }
        for (int i = 0; i < 26; ++i) {
            if (d[i] < target[i]) return false;
        }
        return true;
    }
};

int main() {
    vector<pair<string, vector<string>>> inputs {
        { "1s3 PSt", { "step", "steps", "stripe", "stepple" } },
        { "1s3 456", { "looks", "pest", "stew", "show" } },
    };
    for (pair<string, vector<string>> input : inputs) {
        string ret = Solution().shortestCompletingWord(input.first, input.second);
        cout << ret << endl;
    }
}
