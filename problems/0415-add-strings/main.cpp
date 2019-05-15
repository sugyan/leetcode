#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string addStrings(string num1, string num2) {
        string answer;
        int l = max(num1.length(), num2.length());
        int carry = 0;
        for (int i = 0; i < l; ++i) {
            int d = carry;
            if (i < num1.length()) d += num1[num1.length() - 1 - i] - '0';
            if (i < num2.length()) d += num2[num2.length() - 1 - i] - '0';
            if (d > 9) {
                d -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            answer += '0' + d;
        }
        if (carry) answer += '1';
        reverse(answer.begin(), answer.end());
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
        string num1 = stringToString(line);
        getline(cin, line);
        string num2 = stringToString(line);
        
        string ret = Solution().addStrings(num1, num2);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
