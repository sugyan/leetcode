#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int trailingZeroes(int n) {
        int answer = 0;
        for (long i = 5; i <= n; i *= 5) answer += n / i;
        return answer;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int n = stringToInteger(line);
        
        int ret = Solution().trailingZeroes(n);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
