#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> allCellsDistOrder(int R, int C, int r0, int c0) {
        vector<vector<int>> answer { { r0, c0 } };
        int d = 1;
        while (answer.size() < R * C) {
            int j, r, c;
            for (int i = -d; i <= d; ++i) {
                j = d - abs(i);
                r = r0 + i, c = c0 + j;
                if (r >= 0 && r < R && c >= 0 && c < C) answer.push_back({ r, c });
                if (j != 0) {
                    j *= -1;
                    r = r0 + i, c = c0 + j;
                    if (r >= 0 && r < R && c >= 0 && c < C) answer.push_back({ r, c });
                }
                j = d + abs(i);
            }
            ++d;
        }
        return answer;
    }
};

int main() {
    auto ret = Solution().allCellsDistOrder(2, 3, 1, 2);
    for (auto v : ret) {
        cout << v.front() << "," << v.back() << endl;
    }
}
