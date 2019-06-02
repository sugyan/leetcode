#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<vector<int>> indexPairs(string text, vector<string>& words) {
        vector<pair<int, int>> v;
        for (string word : words) {
            for (int i = 0, tl = text.length(), wl = word.length(); i <= tl - wl; ++i) {
                int pos = text.find(word, i);
                if (pos != string::npos) {
                    v.push_back({ pos, pos + wl - 1 });
                    i = pos;
                }
            }
        }
        sort(v.begin(), v.end());
        vector<vector<int>> answer;
        for (pair<int, int> p : v) {
            answer.push_back(vector<int> { p.first, p.second });
        }
        return answer;
    }
};

int main() {
    string text = "thestoryofleetcodeandme";
    vector<string> words { "story", "fleet", "leetcode" };
    vector<vector<int>> ret = Solution().indexPairs(text, words);
    for (vector<int> p : ret) {
        cout << p[0] << ", " << p[1] << endl;
    }
}
