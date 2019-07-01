#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool validPalindrome(string s) {
        int l = 0, r = s.length() - 1;
        while (l < r) {
            if (s[l] != s[r]) break;
            ++l, --r;
        }
        if (l == r) return true;
        // for (int i = 0; i < 2; ++i) {
        //     int ll = l + 1 - i, rr = r - i;
        //     bool ok = true;
        //     while (ll < rr) {
        //         if (s[ll] != s[rr]) {
        //             ok = false;
        //             break;
        //         }
        //         ++ll, --rr;
        //     }
        //     if (ok) return true;
        // }
        return isPalindromeRange(s, l + 1, r) || isPalindromeRange(s, l, r - 1);
    }
private:
    bool isPalindromeRange(string& s, int l, int r) {
        while (l < r) {
            if (s[l] != s[r]) return false;
            ++l, --r;
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
        
        bool ret = Solution().validPalindrome(s);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
