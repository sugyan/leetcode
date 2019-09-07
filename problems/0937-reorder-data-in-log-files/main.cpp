#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> reorderLogFiles(vector<string>& logs) {
        stable_sort(logs.begin(), logs.end(), [](string a, string b) {
            return isalpha(a[a.find_first_of(" ") + 1]) > isalpha(b[b.find_first_of(" ") + 1]);
        });
        auto it = logs.begin();
        while (isalpha(it->at(it->find_first_of(" ") + 1))) ++it;
        sort(logs.begin(), it, [](string a, string b) {
            string sa = a.substr(a.find_first_of(" ") + 1);
            string sb = b.substr(b.find_first_of(" ") + 1);
            return sa == sb ? a < b : sa < sb;
        });
        return logs;
    }
};

int main() {
    vector<string> logs {
        "dig1 8 1 5 1",
        "let1 art can",
        "dig2 3 6",
        "let2 own kit dig",
        "let3 art zero",
    };
    vector<string> ret = Solution().reorderLogFiles(logs);
    for (string out : ret) {
        cout << out << endl;
    }
}
