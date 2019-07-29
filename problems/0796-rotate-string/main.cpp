#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool rotateString(string A, string B) {
        int l = A.length();
        if (l != B.length()) return false;
        if (A == B) return true;
        for (int i = 0; i < l; ++i) {
            bool ok = true;
            for (int j = 0; j < l; ++j) {
                if (A[j] != B[(i + j) % l]) {
                    ok = false;
                    break;
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
        string A = stringToString(line);
        getline(cin, line);
        string B = stringToString(line);
        
        bool ret = Solution().rotateString(A, B);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
