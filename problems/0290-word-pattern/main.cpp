#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool wordPattern(string pattern, string str) {
        istringstream iss(str);
        unordered_map<string, char> um;
        string word;
        string dict[26];
        int i = 0;
        for(int length = pattern.length(); iss >> word; ++i) {
            if (i >= length) return false;
            int idx = pattern[i] - 'a';
            if (!dict[idx].empty() && dict[idx] != word) return false;
            if (um.find(word) != um.end() && um[word] != pattern[i]) return false;
            um[word] = pattern[i];
            dict[idx] = word;
        }
        if (i != pattern.length()) return false;
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
        string pattern = stringToString(line);
        getline(cin, line);
        string str = stringToString(line);
        
        bool ret = Solution().wordPattern(pattern, str);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
