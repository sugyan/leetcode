#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int findJudge(int N, vector<vector<int>>& trust) {
        vector<int>v(N, 0);
        for (auto& t : trust) {
            --v[t[0] - 1];
            ++v[t[1] - 1];
        }
        for (int i = 0; i < N; ++i) {
            if (v[i] == N - 1) return i + 1;
        }
        return -1;
    }
};

int main() {
    vector<pair<int, vector<vector<int>>>> inputs {
        {
            2,
            {
                { 1, 2 },
            },
        },
        {
            3,
            {
                { 1, 3 },
                { 2, 3 },
            },
        },
        {
            3,
            {
                { 1, 3 },
                { 2, 3 },
                { 3, 1 },
            },
        },
        {
            3,
            {
                { 1, 2 },
                { 2, 3 },
            },
        },
        {
            4,
            {
                { 1, 3 },
                { 1, 4 },
                { 2, 3 },
                { 2, 4 },
                { 4, 3 },
            },
        },
    };
    for (auto p : inputs) {
        int ret = Solution().findJudge(p.first, p.second);
        cout << ret << endl;
    }
}
