#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int hammingWeight(uint32_t n) {
        int answer = 0;
        while (n > 0) {
            ++answer;
            n &= n - 1;
        }
        return answer;
    }
};

uint32_t stringToInteger(string input) {
    return (uint32_t)stoul(input, nullptr, 2);
}

int main() {
    string line;
    while (getline(cin, line)) {
        uint32_t n = stringToInteger(line);
        
        int ret = Solution().hammingWeight(n);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
