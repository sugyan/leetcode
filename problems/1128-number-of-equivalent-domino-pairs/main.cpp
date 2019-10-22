#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numEquivDominoPairs(vector<vector<int>>& dominoes) {
        int m[100] = { 0 };
        for (vector<int>& domino : dominoes) {
            if (domino[0] > domino[1]) {
                ++m[domino[1] * 10 + domino[0]];
            } else {
                ++m[domino[0] * 10 + domino[1]];
            }
        }
        int answer = 0;
        for (int& n : m) {
            if (n > 1) answer += n * (n - 1) / 2;
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> dominoes {
        { 1, 2 }, { 2, 1 }, { 3, 4 }, { 5, 6 },
    };
    int ret = Solution().numEquivDominoPairs(dominoes);
    cout << ret << endl;
}
