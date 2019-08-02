#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    double largestTriangleArea(vector<vector<int>>& points) {
        int size = points.size();
        int maxsize = 0;
        for (int i = 0; i < size - 2; ++i) {
            for (int j = i + 1; j < size - 1; ++j) {
                for (int k = j + 1; k < size; ++k) {
                    int s = abs((points[i][0] - points[k][0]) * (points[j][1] - points[k][1])
                              - (points[j][0] - points[k][0]) * (points[i][1] - points[k][1]));
                    if (s > maxsize) {
                        maxsize = s;
                    }
                }
            }
        }
        return maxsize * 0.5;
    }
};

int main() {
    vector<vector<int>> points {
        { 0, 0 },
        { 0, 1 },
        { 1, 0 },
        { 0, 2 },
        { 2, 0 },
    };
    double ret = Solution().largestTriangleArea(points);
    cout << ret << endl;
}
