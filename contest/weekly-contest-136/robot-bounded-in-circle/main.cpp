#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool isRobotBounded(string instructions) {
        pair<int, int> d = { 0, 1 }, p = { 0, 0 };
        for (int i = 0; i < 4; ++i) {
            for (char c : instructions) {
                if (c == 'G') {
                    p.first += d.first;
                    p.second += d.second;
                }
                if (c == 'L') {
                    d = { - d.second, d.first };
                }
                if (c == 'R') {
                    d = { d.second, - d.first };
                }
            }
        }
        return p.first == 0 && p.second == 0;
    }
};

int main() {
    cout << Solution().isRobotBounded("GL") << endl;
}
