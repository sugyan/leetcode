#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        vector<int> answer;
        if (s.length() < p.length()) return answer;
        int d1[26] { 0 }, d2[26] { 0 };
        for (char c : p) ++d1[c - 'a'];
        for (int i = 0, length = p.length(); i < length; ++i) ++d2[s[i] - 'a'];
        for (int i = 0, sl = s.length(), pl = p.length(); i <= sl - pl; ++i) {
            bool ok = true;
            for (int j = 0; j < 26; ++j) {
                if (d1[j] != d2[j]) {
                    ok = false;
                    break;
                }
            }
            if (ok) answer.push_back(i);
            if (i + pl < sl) {
                --d2[s[i] - 'a'];
                ++d2[s[i + pl] - 'a'];
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

string integerVectorToString(vector<int> list, int length = -1) {
    if (length == -1) {
        length = list.size();
    }

    if (length == 0) {
        return "[]";
    }

    string result;
    for(int index = 0; index < length; index++) {
        int number = list[index];
        result += to_string(number) + ", ";
    }
    return "[" + result.substr(0, result.length() - 2) + "]";
}

int main() {
    string line;
    while (getline(cin, line)) {
        string s = stringToString(line);
        getline(cin, line);
        string p = stringToString(line);
        
        vector<int> ret = Solution().findAnagrams(s, p);

        string out = integerVectorToString(ret);
        cout << out << endl;
    }
    return 0;
}
