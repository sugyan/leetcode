#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string toGoatLatin(string S) {
        string answer;
        bool vowel[26] { 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0 };
        int count = 1;
        for (int i = 0, length = S.length(); i < length;) {
            int j = S.find_first_of(' ', i);
            string word = S.substr(i, j - i);
            if (vowel[tolower(word[0]) - 'a']) {
                answer += word;
            } else {
                answer += word.substr(1);
                answer += word[0];
            }
            answer += "ma";
            for (int k = 0; k < count; ++k) answer += 'a';
            if (j == string::npos) break;
            ++count;
            answer += ' ';
            i = j + 1;
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
        string S = stringToString(line);
        
        string ret = Solution().toGoatLatin(S);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
