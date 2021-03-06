#include <bits/stdc++.h>

using namespace std;

class Solution {
   public:
    string longestPalindrome(string s) {
        if (s.length() < 2) return s;
        int j, l = 0, r = 0;
        for (int i = 0; i < s.length(); ++i) {
            j = 0;
            while (i - j >= 0 && i + 1 + j < s.length() &&
                   s[i - j] == s[i + 1 + j])
                ++j;
            if (j * 2 > r - l) {
                l = i - j + 1, r = i + j + 1;
            }
            j = 0;
            while (i - j >= 0 && i + j < s.length() && s[i - j] == s[i + j])
                ++j;
            if (j * 2 - 1 > r - l) {
                l = i - j + 1, r = i + j;
            }
        }
        return s.substr(l, r - l);
    }
};

string stringToString(string input) {
    assert(input.length() >= 2);
    string result;
    for (int i = 1; i < input.length() - 1; i++) {
        char currentChar = input[i];
        if (input[i] == '\\') {
            char nextChar = input[i + 1];
            switch (nextChar) {
                case '\"':
                    result.push_back('\"');
                    break;
                case '/':
                    result.push_back('/');
                    break;
                case '\\':
                    result.push_back('\\');
                    break;
                case 'b':
                    result.push_back('\b');
                    break;
                case 'f':
                    result.push_back('\f');
                    break;
                case 'r':
                    result.push_back('\r');
                    break;
                case 'n':
                    result.push_back('\n');
                    break;
                case 't':
                    result.push_back('\t');
                    break;
                default:
                    break;
            }
            i++;
        } else {
            result.push_back(currentChar);
        }
    }
    return result;
}

int main() {
    string line;
    while (getline(cin, line)) {
        string s = stringToString(line);

        string ret = Solution().longestPalindrome(s);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
