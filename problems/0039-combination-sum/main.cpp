#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<vector<int>> answer;
        vector<int> combination;
        find(candidates, target, answer, combination);
        return answer;
    }
private:
    void find(vector<int>& candidates, int target, vector<vector<int>>& answer, vector<int>& combination) {
        if (target == 0) answer.push_back(combination);
        for (int candidate : candidates) {
            if (!combination.empty() && candidate < combination.back()) continue;
            if (candidate <= target) {
                combination.push_back(candidate);
                find(candidates, target - candidate, answer, combination);
                combination.pop_back();
            }
        }
    }
};

int main() {
    {
        vector<int> candidates { 2, 3, 6, 7 };
        int target = 7;
        vector<vector<int>> ret = Solution().combinationSum(candidates, target);
        for (vector<int> out : ret) {
            for (int num : out) cout << num << " ";
            cout << endl;
        }
    }
    {
        vector<int> candidates { 2, 3, 5 };
        int target = 8;
        vector<vector<int>> ret = Solution().combinationSum(candidates, target);
        for (vector<int> out : ret) {
            for (int num : out) cout << num << " ";
            cout << endl;
        }
    }
}