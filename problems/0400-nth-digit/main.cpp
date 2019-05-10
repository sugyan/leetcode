#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int findNthDigit(int n) {
        long m = 0;
        int digits = 1;
        while (m < n) {
            m += digits * 9 * pow(10, digits - 1);
            ++digits;
        }
        --digits;
        int target = pow(10, digits) - (m - n) / digits - 1;
        for (int i = 0; i < (m - n) % digits; ++i) target /= 10;
        return target % 10;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int n = stringToInteger(line);
        
        int ret = Solution().findNthDigit(n);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
