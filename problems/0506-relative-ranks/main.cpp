#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<string> findRelativeRanks(vector<int>& nums) {
        unordered_map<int, int> um;
        for (int i = 0, size = nums.size(); i < size; ++i) {
            um[nums[i]] = i;
        }
        sort(nums.rbegin(), nums.rend());
        vector<string> answer(nums.size());
        for (int i = 0, size = nums.size(); i < size; ++i) {
            if (i < 3) {
                if (i == 0) answer[um[nums[i]]] = "Gold Medal";
                if (i == 1) answer[um[nums[i]]] = "Silver Medal";
                if (i == 2) answer[um[nums[i]]] = "Bronze Medal";
            } else {
                answer[um[nums[i]]] = to_string(i + 1);
            }
        }
        return answer;
    }
};

int main() {
    vector<int> nums { 5, 4, 3, 2, 1 };
    vector<string> ret = Solution().findRelativeRanks(nums);
    for (string out : ret) {
        cout << out << endl;
    }
}
