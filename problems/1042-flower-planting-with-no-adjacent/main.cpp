#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> gardenNoAdj(int N, vector<vector<int>>& paths) {
        vector<int> answer(N, 0);
        vector<vector<int>> p(N);
        for (vector<int>& path : paths) {
            p[path[0] - 1].push_back(path[1] - 1);
            p[path[1] - 1].push_back(path[0] - 1);
        }
        for (int i = 0; i < N; ++i) {
            bool c[4] = { true, true, true ,true };
            for (int dst : p[i]) {
                if (answer[dst] > 0) c[answer[dst] - 1] = false;
            }
            for (int j = 0; j < 4; ++j) {
                if (c[j]) {
                    answer[i] = j + 1;
                    break;
                }
            }
        }
        return answer;
    }
};

int main() {
    vector<pair<int, vector<vector<int>>>> inputs {
        {
            3,
            { { 1, 2 }, { 2, 3 }, { 3, 1 } },
        },
        {
            4,
            { { 1, 2 }, { 3, 4 } },
        },
        {
            4,
            { { 1, 2 }, { 2, 3 }, { 3, 4 }, { 4, 1 }, { 1, 3 }, { 2, 4 }},
        },
    };
    for (pair<int, vector<vector<int>>>& input : inputs) {
        vector<int> ret = Solution().gardenNoAdj(input.first, input.second);
        copy(ret.begin(), ret.end(), ostream_iterator<int>(cout, ", "));
        cout << endl;
    }
}
