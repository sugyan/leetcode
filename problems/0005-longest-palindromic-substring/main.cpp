#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string palindrome(string s, int c, bool odd) {
        int i = 0, length = s.length();
        int l = c + (odd ? 1 : 0), r = c;
        while ((l - (i + 1) >= 0 && r + (i + 1) <= length - 1) &&
               (s[l - (i + 1)] == s[r + (i + 1)])) {
            ++i;
        }
        return s.substr(l - i, 2 * i + (odd ? 0 : 1));
    }
    string longestPalindrome(string s) {
        string answer = "", substr;
        for (int i = 0, l = s.length(); i < l; ++i) {
            substr = palindrome(s, i, true);
            if (substr.length() > answer.length()) {
                answer = substr;
            }
            substr = palindrome(s, i, false);
            if (substr.length() > answer.length()) {
                answer = substr;
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
