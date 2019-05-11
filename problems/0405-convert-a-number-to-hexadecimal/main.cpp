#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string toHex(int num) {
        char hex[16] = {
            '0', '1', '2', '3', '4', '5', '6', '7',
            '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        };
        if (num == 0) return "0";
        string answer;
        bool flg = false;
        for (int i = 0; i < 8; ++i) {
            int n = (num >> (28 - i * 4)) & 0x0F;
            if (n > 0) flg = true;
            if (flg) {
                answer += hex[n];
            }
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
        int num = stringToInteger(line);
        
        string ret = Solution().toHex(num);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
