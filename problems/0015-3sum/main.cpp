#include <bits/stdc++.h>

using namespace std;

class Solution {
   public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> answer;
        for (int i = 0, size = nums.size(); i < size; ++i) {
            int l = i + 1, r = size - 1;
            while (l < r) {
                int sum = nums[i] + nums[l] + nums[r];
                if (sum < 0) {
                    ++l;
                } else if (sum > 0) {
                    --r;
                } else {
                    vector<int> v{nums[i], nums[l], nums[r]};
                    answer.push_back(v);
                    while (l < r && nums[l] == v[1]) ++l;
                    while (l < r && nums[r] == v[2]) --r;
                }
            }
            while (i < size - 1 && nums[i + 1] == nums[i]) ++i;
        }
        return answer;
    }
};

int main() {
    vector<int> nums{-1, 0, 1, 2, -1, -4};
    vector<vector<int>> ret = Solution().threeSum(nums);
    vector<string> out;
    transform(ret.begin(), ret.end(), back_inserter(out), [](vector<int> v) {
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
