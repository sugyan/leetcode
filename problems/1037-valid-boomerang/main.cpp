#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool isBoomerang(vector<vector<int>>& points) {
        if ((points[0][0] == points[1][0] && points[0][1] == points[1][1]) ||
            (points[0][0] == points[2][0] && points[0][1] == points[2][1]) ||
            (points[1][0] == points[2][0] && points[1][1] == points[2][1])) return false;
        if (points[0][0] == points[1][0] && points[1][0] == points[2][0]) return false;
        if (points[0][0] == points[1][0] || points[1][0] == points[2][0]) return true;
        double g1 = 1.0 * (points[1][1] - points[0][1]) / (points[1][0] - points[0][0]);
        double g2 = 1.0 * (points[2][1] - points[1][1]) / (points[2][0] - points[1][0]);
        return g1 != g2;
    }
};

int main() {
    vector<vector<vector<int>>> inputs {
        {
            { 1, 1 }, { 2, 3 }, { 3, 2 },
        },
        {
            { 1, 1 }, { 2, 2 }, { 3, 3 },
        },
    };
    for (vector<vector<int>>& points : inputs) {
        bool ret = Solution().isBoomerang(points);
        cout << ret << endl;
    }
}
