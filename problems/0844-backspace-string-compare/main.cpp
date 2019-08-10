#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool backspaceCompare(string S, string T) {
        int s = 0, t = 0;
        for (int i = 0, length = S.length(); i < length; ++i) {
            if (S[i] == '#') s = max(0, s - 1);
            else S[s++] = S[i];
        }
        for (int i = 0, length = T.length(); i < length; ++i) {
            if (T[i] == '#') t = max(0, t - 1);
            else T[t++] = T[i];
        }
        return S.substr(0, s) == T.substr(0, t);
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
        string S = stringToString(line);
        getline(cin, line);
        string T = stringToString(line);
        
        bool ret = Solution().backspaceCompare(S, T);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
