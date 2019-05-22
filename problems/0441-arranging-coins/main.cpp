#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int arrangeCoins(int n) {
        long answer = sqrt(n);
        while ((answer + 1) * (answer + 2) / 2 <= n) ++answer;
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
        
        int ret = Solution().arrangeCoins(n);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
