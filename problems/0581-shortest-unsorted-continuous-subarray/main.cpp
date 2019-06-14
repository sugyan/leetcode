#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int findUnsortedSubarray(vector<int>& nums) {
        int l = 0, r = nums.size() - 1;
        while (l < r && nums[l] <= nums[l + 1]) ++l;
        while (l < r && nums[r] >= nums[r - 1]) --r;
        int vmin = nums[l], vmax = nums[l];
        for (int i = l; i <= r; ++i) {
            vmin = min(vmin, nums[i]);
            vmax = max(vmax, nums[i]);
        }
        while (l > 0 && nums[l - 1] > vmin) --l;
        while (r < nums.size() - 1 && nums[r + 1] < vmax) ++r;
        return l == r ? 0 : r - l + 1;
    }
};

void trimLeftTrailingSpaces(string &input) {
    input.erase(input.begin(), find_if(input.begin(), input.end(), [](int ch) {
        return !isspace(ch);
    }));
}

void trimRightTrailingSpaces(string &input) {
    input.erase(find_if(input.rbegin(), input.rend(), [](int ch) {
        return !isspace(ch);
    }).base(), input.end());
}

vector<int> stringToIntegerVector(string input) {
    vector<int> output;
    trimLeftTrailingSpaces(input);
    trimRightTrailingSpaces(input);
    input = input.substr(1, input.length() - 2);
    stringstream ss;
    ss.str(input);
    string item;
    char delim = ',';
    while (getline(ss, item, delim)) {
        output.push_back(stoi(item));
    }
    return output;
}

int main() {
    string line;
    while (getline(cin, line)) {
        vector<int> nums = stringToIntegerVector(line);
        
        int ret = Solution().findUnsortedSubarray(nums);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
