#include <iostream>
#include <cassert>
#include <cmath>

using namespace std;

class Solution {
public:
    // Boyer–Moore algorithm
    int strStr(string haystack, string needle) {
        int lh = haystack.length(), ln = needle.length();
        if (ln == 0) {
            return 0;
        }
        if (lh < ln) {
            return -1;
        }
        int table[256];
        for (int i = 0; i < 256; ++i) table[i] = ln;
        for (int i = 0; i < ln; ++i) {
            table[needle[i]] = min(table[needle[i]], ln - 1 - i);
        }
        for (int i = 0; i < lh - ln + 1;) {
            int j = ln - 1;
            while (j >= 0 && haystack[i + j] == needle[j]) {
                --j;
            }
            if (j < 0) {
                return i;
            }
            i += max(1, j - ln + table[haystack[i + j]] + 1);
        }
        return -1;
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
        string haystack = stringToString(line);
        getline(cin, line);
        string needle = stringToString(line);
        
        int ret = Solution().strStr(haystack, needle);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
