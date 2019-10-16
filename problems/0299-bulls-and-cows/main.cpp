#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string getHint(string secret, string guess) {
        int a = 0, b = 0;
        int c[10] = { 0 };
        for (int i = 0, len = secret.length(); i < len; ++i) {
            if (secret[i] == guess[i]) ++a;
            else ++c[secret[i] - '0'];
        }
        for (int i = 0, len = guess.length(); i < len; ++i) {
            if (secret[i] != guess[i] && c[guess[i] - '0']-- > 0) ++b;
        }
        return to_string(a) + 'A' + to_string(b) + 'B';
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
        string secret = stringToString(line);
        getline(cin, line);
        string guess = stringToString(line);
        
        string ret = Solution().getHint(secret, guess);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
