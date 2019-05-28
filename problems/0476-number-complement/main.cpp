#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int findComplement(int num) {
        long l = num;
        while ((l & l - 1) > 0) l &= l - 1;
        return (l << 1) - 1 - num;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int num = stringToInteger(line);
        
        int ret = Solution().findComplement(num);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
