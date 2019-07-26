#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int rotatedDigits(int N) {
        bool d1[10] { true, true, true, false, false, true, true, false, true, true };
        bool d2[10] { false, false, true, false, false, true, true, false, false, true };
        int answer = 0;
        for (int i = 1; i <= N; ++i) {
            int n = i;
            bool ok1 = true;
            bool ok2 = false;
            while (n > 0) {
                int d = n % 10;
                if (!d1[d]) {
                    ok1 = false;
                    break;
                }
                if (d2[d]) ok2 = true;
                n /= 10;
            }
            if (ok1 && ok2) ++answer;
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
        
        int ret = Solution().rotatedDigits(N);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
