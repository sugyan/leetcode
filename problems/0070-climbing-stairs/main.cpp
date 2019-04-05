#include <bits/stdc++.h>

using namespace std;

class Solution {
    unordered_map<int, int> memo;
public:
    int climbStairs(int n) {
        if (memo.find(n) != memo.end()) {
            return memo[n];
        }
        int answer;
        if (n <= 1) {
            answer = 1;
        } else {
            answer = climbStairs(n - 1) + climbStairs(n - 2);
        }
        memo[n] = answer;
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
        
        int ret = Solution().climbStairs(n);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
