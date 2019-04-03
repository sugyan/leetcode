#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    // TODO: improve
    string longestPalindrome(string s) {
        string answer = "";
        for (int i = 0, l = s.length(); i < l; ++i) {
            {
                int j = 0;
                while (i - j - 1 >= 0 && i + j + 1 <= l - 1 && s[i - (j + 1)] == s[i + (j + 1)]) {
                    ++j;
                }
                if (2 * j + 1 > answer.length()) {
                    answer = s.substr(i - j, 2 * j + 1);
                }
            }
            {
                int j = 0;
                while (i - j >= 0 && i + j + 1 <= l - 1 && s[i + 1 - (j + 1)] == s[i + (j + 1)]) {
                    ++j;
                }
                if (2 * j > answer.length()) {
                    answer = s.substr(i + 1 - j, 2 * j);
                }
            }
        }
        return answer;
    }
};

string stringToString(string input) {
    assert(input.length() >= 2);
    string result;
    for (int i = 1; i < input.length() -1; i++) {
        char currentChar = input[i];
        if (input[i] == '\\') {
            char nextChar = input[i+1];
            switch (nextChar) {
                case '\"': result.push_back('\"'); break;
                case '/' : result.push_back('/'); break;
                case '\\': result.push_back('\\'); break;
                case 'b' : result.push_back('\b'); break;
                case 'f' : result.push_back('\f'); break;
                case 'r' : result.push_back('\r'); break;
                case 'n' : result.push_back('\n'); break;
                case 't' : result.push_back('\t'); break;
                default: break;
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
