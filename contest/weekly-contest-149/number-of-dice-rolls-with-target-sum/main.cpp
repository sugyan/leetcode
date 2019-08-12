#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numRollsToTarget(int d, int f, int target) {
        if (d == 0 || target < d || target > d * f) return 0;
        if (d == 1 && target <= f) return 1;
        long sum = 0;
        string key = to_string(d) + "-" + to_string(target);
        if (memo.find(key) != memo.end()) {
            sum = memo[key];
        } else {
            for (int i = 1; i <= f; ++i) {
                sum += numRollsToTarget(d - 1, f, target - i);
            }
            memo[key] = sum % mod;
        }
        return sum % mod;
    }
private:
    unordered_map<string, int> memo;
    long answer = 0;
    long mod = 1000000007;
};

int main() {
    vector<vector<int>> inputs {
        { 1, 6, 3 },
        { 2, 6, 7 },
        { 2, 5, 10 },
        { 1, 2, 3 },
        { 30, 30, 500 },
    };
    for (vector<int> input : inputs) {
        int ret = Solution().numRollsToTarget(input[0], input[1], input[2]);
        cout << ret << endl;
    }
}
