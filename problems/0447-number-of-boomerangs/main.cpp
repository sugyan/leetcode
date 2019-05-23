#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numberOfBoomerangs(vector<vector<int>>& points) {
        int size = points.size();
        vector<vector<int>> v(size, vector<int>(size, 0));
        for (int i = 0; i < size - 1; ++i) {
            for (int j = i + 1; j < size; ++j) {
                int dx = points[i][0] - points[j][0];
                int dy = points[i][1] - points[j][1];
                v[i][j] = v[j][i] = dx * dx + dy * dy;
            }
        }
        int answer = 0;
        unordered_map<int, int> um;
        for (int i = 0; i < size; ++i) {
            um.clear();
            for (int j = 0; j < size; ++j) {
                ++um[v[i][j]];
            }
            for (pair<int, int> p : um) {
                if (p.second > 1) {
                    answer += p.second * (p.second - 1);
                }
            }
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> points {
        { 0, 0 },
        { 1, 0 },
        { 2, 0 },
    };
    int ret = Solution().numberOfBoomerangs(points);
    cout << ret << endl;
}
