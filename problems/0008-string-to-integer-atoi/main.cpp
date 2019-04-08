#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int myAtoi(string str) {
        long max = numeric_limits<int>::max();
        long min = numeric_limits<int>::min();
        long answer = 0;
        int sign = 1;
        int i = str.find_first_not_of(' ');
        if (i == string::npos) return 0;
        if (str[i] == '+' || str[i] == '-') {
            if (str[i] == '-') sign = -1;
            ++i;
        }
        for (int l = str.length(); i < l && str[i] >= '0' && str[i] <= '9'; ++i) {
            answer = answer * 10 + (str[i] - '0') * sign;
            if (answer > max) {
                return max;
            }
            if (answer < min) {
                return min;
            }
        }
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
        string str = stringToString(line);
        
        int ret = Solution().myAtoi(str);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
