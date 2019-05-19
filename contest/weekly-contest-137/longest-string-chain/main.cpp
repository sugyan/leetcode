#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int longestStrChain(vector<string>& words) {
        int size = words.size();
        for (int i = 0; i < size - 1; ++i) {
            for (int j = i + 1; j < size; ++j) {
                if (words[i].length() > words[j].length()) {
                    swap(words[i], words[j]);
                }
            }
        }
        vector<vector<int>> v(size);
        for (int i = 0; i < size - 1; ++i) {
            for (int j = i + 1; j < size; ++j) {
                if (words[i].length() + 1 == words[j].length()) {
                    bool flg = false;
                    for (int k = 0; k < words[j].length(); ++k) {
                        string word = words[j];
                        word.erase(word.begin() + k);
                        if (word == words[i]) {
                            flg = true;
                            break;
                        }
                    }
                    if (flg) {
                        v[i].push_back(j);
                    }
                }
            }
        }
        int answer = 0;
        for (int i = 0; i < size; ++i) {
            answer = max(answer, depth(v, i, 1));
        }
        return answer;
    }
private:
    int depth(vector<vector<int>>& v, int idx, int d) {
        if (v[idx].empty()) return d;
        int answer = 0;
        for (int next : v[idx]) {
            answer = max(answer, depth(v, next, d + 1));
        }
        return answer;
    }
};

int main() {
    vector<string> words { "a", "b", "ba", "bca", "bda", "bdca" };
    int ret = Solution().longestStrChain(words);
    cout << ret << endl;
}
