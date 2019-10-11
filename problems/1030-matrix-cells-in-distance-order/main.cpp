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
                    c = c0 - j;
                    if (r >= 0 && r < R && c >= 0 && c < C) answer.push_back({ r, c });
                }
            }
            ++d;
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> inputs {
        { 1, 2, 0, 0 },
        { 2, 2, 0, 1 },
        { 2, 3, 1, 2 },
    };
    for (vector<int>& in : inputs) {
        vector<vector<int>> ret = Solution().allCellsDistOrder(in[0], in[1], in[2], in[3]);
        vector<string> out;
        transform(ret.begin(), ret.end(), back_inserter(out), [](vector<int> cell) {
            return "[" + to_string(cell[0]) + ", " + to_string(cell[1]) + "]";
        });
        copy(out.begin(), out.end(), ostream_iterator<string>(cout, ", "));
        cout << endl;
    }
}
