#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> answer;
        for (int i = 0, size = nums.size(); i < size - 3; ++i) {
            // Optimization
            if (nums[i] + nums[i + 1] + nums[i + 2] + nums[i + 3] > target) break;
            if (nums[i] + nums[size - 3] + nums[size - 2] + nums[size - 1] < target) continue;
            for (int j = i + 1; j < size - 2; ++j) {
                // Optimization
                if (nums[i] + nums[j] + nums[j + 1] + nums[j + 2] > target) break;
                if (nums[i] + nums[j] + nums[size - 2] + nums[size - 1] < target) continue;
                int l = j + 1, r = size - 1;
                while (l < r) {
                    int sum = nums[i] + nums[j] + nums[l] + nums[r];
                    if (sum < target) {
                        ++l;
                    } else if (sum > target) {
                        --r;
                    } else {
                        vector<int> v { nums[i], nums[j], nums[l], nums[r] };
                        answer.push_back(v);
                        while (l < r && nums[l] == v[2]) ++l;
                        while (l < r && nums[r] == v[3]) --r;
                    }
                }
                while (j < size - 2 && nums[j + 1] == nums[j]) ++j;
            }
            while (i < size - 3 && nums[i + 1] == nums[i]) ++i;
        }
        return answer;
    }
};

string integerVectorToString(vector<int> list, int length = -1) {
    if (length == -1) {
        length = list.size();
    }

    if (length == 0) {
        return "[]";
    }

    string result;
    for(int index = 0; index < length; index++) {
        int number = list[index];
        result += to_string(number) + ", ";
    }
    return "[" + result.substr(0, result.length() - 2) + "]";
}

int main() {
    vector<int> nums { 1, 0, -1, 0, -2, 2 };
    vector<vector<int>> ret = Solution().fourSum(nums, 0);
    for (auto v : ret) {
        string out = integerVectorToString(v);
        cout << out << endl;
    }
}
