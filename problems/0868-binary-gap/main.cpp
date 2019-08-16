#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int binaryGap(int N) {
        int answer = 0;
        int length = 0;
        while (N > 0) {
            if (N & 1) {
                answer = max(answer, length);
                length = 1;
            } else if (length > 0) {
                ++length;
            };
            N >>= 1;
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
        int N = stringToInteger(line);
        
        int ret = Solution().binaryGap(N);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
