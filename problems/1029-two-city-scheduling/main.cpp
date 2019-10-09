#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int twoCitySchedCost(vector<vector<int>>& costs) {
        int answer = 0, sum = 0;
        vector<int> diff(costs.size());
        for (int i = 0, size = costs.size(); i < size; ++i) {
            diff[i] = costs[i][0] - costs[i][1];
            answer += costs[i][0];
        }
        sort(diff.begin(), diff.end(), greater<int>());
        for (int i = 0, size = costs.size() / 2; i < size; ++i) {
            answer -= diff[i];
        }
        return answer;
    }
};

int main() {
    vector<vector<int>> costs {
        {  10,  20 },
        {  30, 200 },
        { 400,  50 },
        {  30,  20 },
    };
    int ret = Solution().twoCitySchedCost(costs);
    cout << ret << endl;
}
