#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int reachNumber(int target) {
        target = abs(target);
        int a = sqrt(target);
        while (a * (a + 1) / 2 < target || (a * (a + 1) / 2 - target) % 2 != 0) ++a;
        return a;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int target = stringToInteger(line);
        
        int ret = Solution().reachNumber(target);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
