#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int addDigits(int num) {
        return 1 + (num - 1) % 9;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int num = stringToInteger(line);
        
        int ret = Solution().addDigits(num);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
