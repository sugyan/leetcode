#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string mostCommonWord(string paragraph, vector<string>& banned) {
        unordered_map<string, int> um;
        for (char& c : paragraph) c = tolower(c);
        for (int i = 0, length = paragraph.length(); i < length;) {
            int j = i;
            while (j < length && isalpha(paragraph[j])) ++j;
            ++um[paragraph.substr(i, j - i)];
            while (j < length && !isalpha(paragraph[j])) ++j;
            i = j;
        }
        for (string& word : banned) um[word] = 0;
        string answer;
        int m = 0;
        for (pair<string, int> p : um) {
            if (p.second > m) {
                m = p.second;
                answer = p.first;
            }
        }
        return answer;
    }
};

int main() {
    string paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.";
    vector<string> banned { "hit" };
    string ret = Solution().mostCommonWord(paragraph, banned);
    cout << ret << endl;
}
