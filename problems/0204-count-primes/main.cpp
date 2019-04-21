#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int countPrimes(int n) {
        if (n < 3) return 0;
        vector<int> primes { 2 };
        for (int i = 3; i < n; i += 2) {
            bool ok = true;
            for (int prime : primes) {
                if (prime * prime > i) break;
                if (i % prime == 0) {
                    ok = false;
                    break;
                }
            }
            if (ok) primes.push_back(i);
        }
        return primes.size();
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int n = stringToInteger(line);
        
        int ret = Solution().countPrimes(n);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
