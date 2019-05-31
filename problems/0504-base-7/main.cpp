#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string convertToBase7(int num) {
        if (num < 0) return '-' + convertToBase7(-num);
        if (num < 7) return to_string(num % 7);
        return convertToBase7(num / 7) + to_string(num % 7);
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int num = stringToInteger(line);
        
        string ret = Solution().convertToBase7(num);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
