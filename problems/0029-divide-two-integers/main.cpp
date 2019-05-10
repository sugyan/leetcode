#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int divide(int dividend, int divisor) {
        bool sign = (dividend ^ divisor) >= 0;
        long ldividend = labs(dividend), ldivisor = labs(divisor);
        if (ldividend < ldivisor) return 0;
        long answer = 1;
        long d = ldivisor;
        while (ldividend > d << 1) {
            d <<= 1;
            answer <<= 1;
        }
        if (ldividend > d) {
            answer += divide(ldividend - d, ldivisor);
        }
        if (!sign) answer -= answer + answer;
        if (answer > numeric_limits<int>::max()) {
            answer = numeric_limits<int>::max();
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
        int dividend = stringToInteger(line);
        getline(cin, line);
        int divisor = stringToInteger(line);
        
        int ret = Solution().divide(dividend, divisor);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
