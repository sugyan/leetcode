#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool checkStraightLine(vector<vector<int>>& coordinates) {
        if (coordinates.size() <= 2) return true;
        pair<int, int> d0 = {
            coordinates[1][0] - coordinates[0][0],
            coordinates[1][1] - coordinates[0][1],
        };
        for (int i = 2; i < coordinates.size(); ++i) {
            pair<int, int> d = {
                coordinates[i][0] - coordinates[0][0],
                coordinates[i][1] - coordinates[0][1],
            };
            if (d0.first * d.second != d.first * d0.second) return false;
        }
        return true;
    }
};

int main() {
    vector<vector<vector<int>>> inputs {
        { { 1, 2 }, { 2, 3 }, { 3, 4 }, { 4, 5 }, { 5, 6 }, { 6, 7 } },
        { { 1, 1 }, { 2, 2 }, { 3, 4 }, { 4, 5 }, { 5, 6 }, { 7, 7 } },
    };
    for (vector<vector<int>>& coordinates : inputs) {
        bool ret = Solution().checkStraightLine(coordinates);
        cout << ret << endl;
    }
}
