#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int dayOfYear(string date) {
        int y = stoi(date.substr(0, 4));
        int m = stoi(date.substr(5, 2));
        int d = stoi(date.substr(8, 2));
        int answer = d;
        if (m > 1) answer += 31;
        if (m > 2) {
            answer += 28;
            if (y != 1900 && y % 4 == 0) ++answer;
        }
        if (m > 3) answer += 31;
        if (m > 4) answer += 30;
        if (m > 5) answer += 31;
        if (m > 6) answer += 30;
        if (m > 7) answer += 31;
        if (m > 8) answer += 31;
        if (m > 9) answer += 30;
        if (m > 10) answer += 31;
        if (m > 11) answer += 30;
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
        string date = stringToString(line);
        
        int ret = Solution().dayOfYear(date);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
