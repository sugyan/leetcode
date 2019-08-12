#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int dayOfYear(string date) {
        int y = stoi(date.substr(0, 4));
        int m = stoi(date.substr(5, 2));
        int d = stoi(date.substr(8, 2));
        int answer = d;
        if (m > 1) answer += 31;
        if (m > 2) {
            answer += 28;
            if (y != 1900 && y % 4 == 0) ++answer;
        }
        if (m > 3) answer += 31;
        if (m > 4) answer += 30;
        if (m > 5) answer += 31;
        if (m > 6) answer += 30;
        if (m > 7) answer += 31;
        if (m > 8) answer += 31;
        if (m > 9) answer += 30;
        if (m > 10) answer += 31;
        if (m > 11) answer += 30;
        return answer;
    }
};

int main() {
    vector<string> inputs {
        "2019-01-09",
        "2019-02-10",
        "2003-03-01",
        "2004-03-01",
    };
    for (string date : inputs) {
        int ret = Solution().dayOfYear(date);
        cout << ret << endl;
    }
}
