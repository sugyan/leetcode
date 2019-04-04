#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int mySqrt(int x) {
        if (x <= 1) return x;
        long l = 0, r = x;
        while (l != r) {
            long m = (l + r) / 2;
            if (m * m <= x) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        return l - 1;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int x = stringToInteger(line);
        
        int ret = Solution().mySqrt(x);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
