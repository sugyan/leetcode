#include <bits/stdc++.h>

using namespace std;

#define MOD 1000000007

class Solution {
public:
    int numPrimeArrangements(int n) {
        vector<int> primes {
             2,  3,  5,  7, 11,
            13, 17, 19, 23, 29,
            31, 37, 41, 43, 47,
            53, 59, 61, 67, 71,
            73, 79, 83, 89, 97,
        };
        int m = upper_bound(primes.begin(), primes.end(), n) - primes.begin();
        long long answer = 1;
        for (int i = 2; i <= max(m, n - m); ++i) {
            if (i <= m) answer *= i;
            if (i <= n - m) answer *= i;
            answer %= MOD;
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
        int n = stringToInteger(line);
        
        int ret = Solution().numPrimeArrangements(n);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
