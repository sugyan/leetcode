#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int fib(int N) {
        int mem[31] { 0, 1 };
        for (int i = 2; i <= N; ++i) {
            mem[i] = mem[i - 1] + mem[i - 2];
        }
        return mem[N];
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int N = stringToInteger(line);
        
        int ret = Solution().fib(N);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
