#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool judgeSquareSum(int c) {
        for (int i = 0; i <= sqrt(c); ++i) {
            int d = sqrt(c - i * i);
            if (d * d + i * i == c) return true;
        }
        return false;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

string boolToString(bool input) {
    return input ? "True" : "False";
}

int main() {
    string line;
    while (getline(cin, line)) {
        int c = stringToInteger(line);
        
        bool ret = Solution().judgeSquareSum(c);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
