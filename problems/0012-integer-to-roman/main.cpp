#include <bits/stdc++.h>

using namespace std;

class Solution {
    vector<pair<int, string>> dict {
        { 1000, "M"  },
        {  900, "CM" },
        {  500, "D"  },
        {  400, "CD" },
        {  100, "C"  },
        {   90, "XC" },
        {   50, "L"  },
        {   40, "XL" },
        {   10, "X"  },
        {    9, "IX" },
        {    5, "V"  },
        {    4, "IV" },
        {    1, "I"  },
    };
public:
    string intToRoman(int num) {
        string answer = "";
        while (num > 0) {
            for (auto p : dict) {
                if (p.first <= num) {
                    answer += p.second;
                    num -= p.first;
                    break;
                }
            }
        }
        return answer;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int num = stringToInteger(line);
        
        string ret = Solution().intToRoman(num);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
