#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool isPowerOfFour(int num) {
        if (num < 1) return false;
        return (num & num - 1) == 0 && (num - 1) % 3 == 0;
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
        int num = stringToInteger(line);
        
        bool ret = Solution().isPowerOfFour(num);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
