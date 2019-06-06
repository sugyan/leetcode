#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string reverseStr(string s, int k) {
        for (int i = 0, length = s.length(); i < length; i += 2 * k) {
            int l = min(k, length - i);
            for (int j = 0; j < l / 2; ++j) {
                swap(s[i + j], s[i + l - 1 - j]);
            }
        }
        return s;
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

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        string s = stringToString(line);
        getline(cin, line);
        int k = stringToInteger(line);
        
        string ret = Solution().reverseStr(s, k);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
