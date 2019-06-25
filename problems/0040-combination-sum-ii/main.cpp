#include <bits/stdc++.h>
#include <experimental/iterator>

using namespace std;

class Solution {
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());
        vector<vector<int>> answer;
        vector<int> combination;
        find(candidates, target, answer, combination, 0);
        return answer;
    }
private:
    void find(vector<int>& candidates, int target, vector<vector<int>>& answer, vector<int>& combination, int pos) {
        if (target == 0) {
            answer.push_back(combination);
            return;
        }
        for (int i = pos, size = candidates.size(); i < size; ++i) {
            int candidate = candidates[i];
            if (candidate > target) return;
            if (i > pos && candidate == candidates[i - 1]) continue;
            combination.push_back(candidate);
            find(candidates, target - candidate, answer, combination, i + 1);
            combination.pop_back();
        }
    }
};

int main() {
    vector<pair<vector<int>, int>> inputs {
        { { 10, 1, 2, 7, 6, 1, 5 }, 8 },
        { { 2, 5, 2, 1, 2 }, 5 },
    };
    for (pair<vector<int>, int> input : inputs) {
        vector<vector<int>> ret = Solution().combinationSum2(input.first, input.second);
        for (vector<int> out : ret) {
            copy(out.begin(), out.end(), experimental::make_ostream_joiner(cout, ","));
            cout << " ";
        }
        cout << endl;
    }
}
