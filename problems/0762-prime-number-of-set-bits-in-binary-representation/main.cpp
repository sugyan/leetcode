#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int countPrimeSetBits(int L, int R) {
        set<int> primes { 2, 3, 5, 7, 11, 13, 17, 19 };
        int answer = 0;
        for (int i = L; i <= R; ++i) {
            int n = i;
            int d = 0;
            while (n > 0) {
                if (n & 1) ++d;
                n >>= 1;
            }
            if (primes.find(d) != primes.end()) ++answer;
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
        int L = stringToInteger(line);
        getline(cin, line);
        int R = stringToInteger(line);
        
        int ret = Solution().countPrimeSetBits(L, R);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
