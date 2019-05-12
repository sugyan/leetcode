#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> gardenNoAdj(int N, vector<vector<int>>& paths) {
        vector<int> answer(N);
        for (int i = 0; i < N; ++i) answer[i] = 0;
        vector<vector<int>> p(N);
        for (vector<int> path : paths) {
            p[path[0] - 1].push_back(path[1] - 1);
            p[path[1] - 1].push_back(path[0] - 1);
        }
        for (int i = 0; i < N; ++i) {
            bool c[4] = { true, true, true ,true };
            for (int dst : p[i]) {
                if (answer[dst] > 0) c[answer[dst] - 1] = false;
            }
            int j = 0;
            for (; j < 4; ++j) {
                if (c[j] == true) break;
            }
            answer[i] = j + 1;
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> paths = {
        { 1, 2 },
        { 2, 3 },
        { 3, 4 },
        { 4, 1 },
        { 1, 3 },
        { 2, 4 },
    };
    vector<int> ret = Solution().gardenNoAdj(4, paths);
    for (int out : ret) {
        cout << out << endl;
    }
}
