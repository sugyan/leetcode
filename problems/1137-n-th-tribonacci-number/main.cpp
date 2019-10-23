#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int tribonacci(int n) {
        int answer[38] { 0 };
        answer[0] = 0;
        answer[1] = 1;
        answer[2] = 1;
        for (int i = 3; i <= n; ++i) {
            answer[i] = answer[i - 1] + answer[i - 2] + answer[i - 3];
        }
        return answer[n];
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int n = stringToInteger(line);
        
        int ret = Solution().tribonacci(n);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
