#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> findOcurrences(string text, string first, string second) {
        vector<string> words;
        istringstream iss(text);
        string s;
        while (iss >> s) words.push_back(s);
        vector<string> answer;
        for (int i = 0; i < words.size() - 2; ++i) {
            if (words[i] == first && words[i + 1] == second) answer.push_back(words[i + 2]);
        }
        return answer;
    }
};

int main() {
    vector<vector<string>> inputs {
        {
            "alice is a good girl she is a good student",
            "a",
            "good"
        },
        {
            "we will we will rock you",
            "we",
            "will",
        },
    };
    for (vector<string>& input : inputs) {
        vector<string> ret = Solution().findOcurrences(input[0], input[1], input[2]);
        copy(ret.begin(), ret.end(), ostream_iterator<string>(cout, ", "));
        cout << endl;
    }
}
