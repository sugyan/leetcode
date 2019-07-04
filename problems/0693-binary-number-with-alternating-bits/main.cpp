#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool hasAlternatingBits(int n) {
        long m = n >> 1;
        return ((m + n) & (m + n + 1)) == 0;
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
        int n = stringToInteger(line);
        
        bool ret = Solution().hasAlternatingBits(n);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
