#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string convertToTitle(int n) {
        string answer = "";
        while (n > 0) {
            int m = (n - 1) % 26;
            answer = (char)('A' + m) + answer;
            n -= m - 1;
            n /= 26;
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
        
        string ret = Solution().convertToTitle(n);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
