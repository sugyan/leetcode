#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string addBinary(string a, string b) {
        string answer = "";
        bool carry = false;
        for (int i = 0, la = a.length(), lb = b.length(); i < la || i < lb; ++i) {
            int d = 0;
            if (carry) ++d;
            if (i < la && a[la - i - 1] == '1') ++d;
            if (i < lb && b[lb - i - 1] == '1') ++d;
            if (d > 1) {
                d -= 2;
                carry = true;
            } else {
                carry = false;
            }
            answer = to_string(d) + answer;
        }
        if (carry) answer = '1' + answer;
        return answer;
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
        string a = stringToString(line);
        getline(cin, line);
        string b = stringToString(line);
        
        string ret = Solution().addBinary(a, b);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
