#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int bitwiseComplement(int N) {
        if (N <= 1) return 1 - N;
        int n = 1;
        while (n < N) n <<= 1;
        return n == N ? n - 1 : n - 1 - N;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int N = stringToInteger(line);
        
        int ret = Solution().bitwiseComplement(N);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
