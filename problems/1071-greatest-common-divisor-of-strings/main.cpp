#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string gcdOfStrings(string str1, string str2) {
        if (str2.length() > str1.length()) return gcdOfStrings(str2, str1);
        if (str2 == "") return str1;
        if (str1.substr(0, str2.size()) != str2) return "";
        return gcdOfStrings(str2, str1.substr(str2.size()));
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
        string str1 = stringToString(line);
        getline(cin, line);
        string str2 = stringToString(line);
        
        string ret = Solution().gcdOfStrings(str1, str2);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
