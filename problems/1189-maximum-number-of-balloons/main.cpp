#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int maxNumberOfBalloons(string text) {
        int d[26] { 0 };
        for (char& c : text) ++d[c - 'a'];
        int answer = numeric_limits<int>::max();
        answer = min(answer, d['b' - 'a']);
        answer = min(answer, d['a' - 'a']);
        answer = min(answer, d['l' - 'a'] / 2);
        answer = min(answer, d['o' - 'a'] / 2);
        answer = min(answer, d['n' - 'a']);
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
        string text = stringToString(line);
        
        int ret = Solution().maxNumberOfBalloons(text);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
