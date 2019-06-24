#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int maximumProduct(vector<int>& nums) {
        // sort(nums.rbegin(), nums.rend());
        // return max(nums[0] * nums[1] * nums[2], nums[0] * nums[nums.size() - 1] * nums[nums.size() - 2]);
        int max1, max2, max3, min1, min2;
        max1 = max2 = max3 = numeric_limits<int>::min();
        min1 = min2 = numeric_limits<int>::max();
        for (int num : nums) {
            if (num > max1) max3 = max2, max2 = max1, max1 = num;
            else if (num > max2) max3 = max2, max2 = num;
            else if (num > max3) max3 = num;
            if (num < min1) min2 = min1, min1 = num;
            else if (num < min2) min2 = num;
        }
        return max(max1 * max2 * max3, max1 * min1 * min2);
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
        
        int ret = Solution().maximumProduct(nums);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
