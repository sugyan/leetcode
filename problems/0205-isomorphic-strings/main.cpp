#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool isIsomorphic(string s, string t) {
        char map1[128] = { 0 }, map2[128] = { 0 };
        for (int i = 0, length = s.length(); i < length; ++i) {
            if (map1[s[i]] != map2[t[i]]) return false;
            map1[s[i]] = i + 1;
            map2[t[i]] = i + 1;
        }
        return true;
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
        getline(cin, line);
        string t = stringToString(line);
        
        bool ret = Solution().isIsomorphic(s, t);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
