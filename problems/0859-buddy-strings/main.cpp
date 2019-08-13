#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool buddyStrings(string A, string B) {
        if (A.length() != B.length()) return false;
        if (A == B) {
            int d[26] { 0 };
            for (char& c : A) {
                if (++d[c - 'a'] > 1) return true;
            }
            return false;

        }
        int d1 = -1, d2 = -1;
        for (int i = 0, length = A.length(); i < length; ++i) {
            if (A[i] != B[i]) {
                if (d1 < 0) d1 = i;
                else if (d2 < 0) d2 = i;
                else return false;
            }
        }
        return A[d1] == B[d2] && A[d2] == B[d1];
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
        string A = stringToString(line);
        getline(cin, line);
        string B = stringToString(line);
        
        bool ret = Solution().buddyStrings(A, B);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
