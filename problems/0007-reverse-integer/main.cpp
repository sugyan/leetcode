#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int reverse(int x) {
        long long answer = 0;
        while (x != 0) {
            answer = answer * 10 + x % 10;
            x /= 10;
        }
        if (answer > numeric_limits<int>::max() || answer < numeric_limits<int>::min()) return 0;
        return answer;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int x = stringToInteger(line);
        
        int ret = Solution().reverse(x);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
