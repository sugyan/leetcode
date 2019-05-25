#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool repeatedSubstringPattern(string s) {
        int length = s.length();
        for (int i = 1; i <= length / 2; ++i) {
            if (length % i != 0) continue;
            bool ok = true;
            for (int j = 0; j < i; ++j) {
                for (int k = 1; k < length / i; ++k) {
                    if (s[j + k * i] != s[j]) {
                        ok = false;
                        break;
                    }
                    if (!ok) break;
                }
            }
            if (ok) return true;
        }
        return false;
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

string boolToString(bool input) {
    return input ? "True" : "False";
}

int main() {
    string line;
    while (getline(cin, line)) {
        string s = stringToString(line);
        
        bool ret = Solution().repeatedSubstringPattern(s);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
