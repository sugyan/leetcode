#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int maxRepOpt1(string text) {
        vector<pair<char, int>> v;
        int count = 1;
        for (int i = 1, length = text.length(); i < length; ++i) {
            if (text[i] == text[i - 1]) ++count;
            else {
                v.push_back({ text[i - 1], count });
                count = 1;
            }
        }
        v.push_back({ text.back(), count });
        int answer = 0;
        for (int i = 0, size = v.size(); i < size; ++i) {
            int candidate = v[i].second;
            if (i + 2 < size && v[i + 1].second == 1 && v[i + 2].first == v[i].first) {
                candidate += v[i + 2].second;
                bool exist = false;
                for (int j = 0; j < size; ++j) {
                    if (j != i && j != i + 2 && v[j].first == v[i].first) {
                        exist = true;
                        break;
                    }
                }
                if (exist) ++candidate;
            } else {
                bool exist = false;
                for (int j = 0; j < size; ++j) {
                    if (j != i && v[j].first == v[i].first) {
                        exist = true;
                        break;
                    }
                }
                if (exist) ++candidate;
            }
            answer = max(answer, candidate);
        }
        return answer;
    }
};

int main() {
    vector<string> inputs {
        "ababa",
        "aaabaaa",
        "aaabbaaa",
        "aaaaa",
        "abcdef",
    };
    for (string text : inputs) {
        int ret = Solution().maxRepOpt1(text);
        cout << ret << endl;
    }
}
