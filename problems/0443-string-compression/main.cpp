#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int compress(vector<char>& chars) {
        int answer = 0;
        for (int i = 0, size = chars.size(); i < size; ++i) {
            chars[answer++] = chars[i];
            if (i + 1 < size && chars[i + 1] == chars[i]) {
                int count = 2;
                while (i + count < size && chars[i + count] == chars[i]) ++count;
                string s = to_string(count);
                for (char c : s) chars[answer++] = c;
                i += count - 1;
            }
        }
        return answer;
    }
};

int main() {
    vector<string> inputs {
        "aabbccc",
        "a",
        "abbbbbbbbbbbb",
    };
    for (string input : inputs) {
        vector<char> chars(input.begin(), input.end());
        int ret = Solution().compress(chars);
        for (int i = 0; i < ret; ++i) cout << chars[i];
        cout << endl;
    }
}
