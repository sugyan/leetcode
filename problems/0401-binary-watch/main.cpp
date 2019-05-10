#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> readBinaryWatch(int num) {
        vector<string> answer;
        for (int i = 0; i <= num; ++i) {
            if (i <= 4 && num - i <= 6) {
                vector<int> vh, vm;
                generateHours(vh, i, 0, 0);
                generateMinutes(vm, num - i, 0, 0);
                for (int h : vh) {
                    for (int m : vm) {
                        string s = to_string(h) + ":";
                        if (m < 10) s += '0';
                        s += to_string(m);
                        answer.push_back(s);
                    }
                }
            }
        }
        return answer;
    }
private:
    void generateHours(vector<int>& ret, int remain, int digit, int num) {
        if (remain == 0) {
            if (num < 12) ret.push_back(num);
            return;
        }
        if (digit < 4) {
            generateHours(ret, remain, digit + 1, num);
            generateHours(ret, remain - 1, digit + 1, num + (1 << digit));
        }
    }
    void generateMinutes(vector<int>& ret, int remain, int digit, int num) {
        if (remain == 0) {
            if (num < 60) ret.push_back(num);
            return;
        }
        if (digit < 6) {
            generateMinutes(ret, remain, digit + 1, num);
            generateMinutes(ret, remain - 1, digit + 1, num + (1 << digit));
        }
    }
};

int main() {
    vector<string> ret = Solution().readBinaryWatch(1);
    for (string out : ret) {
        cout << out << endl;
    }
}
