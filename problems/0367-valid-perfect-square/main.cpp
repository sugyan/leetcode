#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool isPerfectSquare(int num) {
        if (num < 1) return false;
        long l = 1, r = num;
        while (r - l > 1) {
            long m = (l + r) / 2;
            if (m * m < num) {
                l = m;
            } else {
                r = m;
            }
        }
        return r * r == num;
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
        
        bool ret = Solution().isPerfectSquare(num);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
