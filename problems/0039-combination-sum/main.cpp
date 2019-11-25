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
    void find(vector<int>& candidates, int target, vector<vector<int>>& answer,
              vector<int>& combination) {
        if (target == 0) answer.push_back(combination);
        for (int candidate : candidates) {
            if (!combination.empty() && candidate < combination.back())
                continue;
            if (candidate <= target) {
                combination.push_back(candidate);
                find(candidates, target - candidate, answer, combination);
                combination.pop_back();
            }
        }
    }
};

int main() {
    vector<pair<vector<int>, int>> inputs{
        {{2, 3, 6, 7}, 7},
        {{2, 3, 5}, 8},
    };
    for (auto input : inputs) {
        vector<vector<int>> ret =
            Solution().combinationSum(input.first, input.second);
        vector<string> out;
        transform(ret.begin(), ret.end(), back_inserter(out),
                  [](vector<int> v) {
                      ostringstream oss;
                      for (auto it = v.begin(); it != v.end(); ++it) {
                          oss << *it;
                          if (it + 1 != v.end()) oss << ", ";
                      }
                      return "[" + oss.str() + "]";
                  });
        copy(out.begin(), out.end(), ostream_iterator<string>(cout, " "));
        cout << endl;
    }
}
