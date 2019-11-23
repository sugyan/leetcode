#include <bits/stdc++.h>

using namespace std;

class Solution {
   public:
    int search(vector<int> &nums, int target) {
        int l = 0, r = nums.size();
        while (l < r) {
            int m = (l + r) / 2;
            int n = nums[m];
            if (n == target) return m;
            if ((n < nums[0]) != (target < nums[0])) {
                n = target < nums[0] ? numeric_limits<int>::min()
                                     : numeric_limits<int>::max();
            }
            if (n > target) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        return -1;
    }
};

void trimLeftTrailingSpaces(string &input) {
    input.erase(input.begin(), find_if(input.begin(), input.end(),
                                       [](int ch) { return !isspace(ch); }));
}

void trimRightTrailingSpaces(string &input) {
    input.erase(find_if(input.rbegin(), input.rend(),
                        [](int ch) { return !isspace(ch); })
                    .base(),
                input.end());
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

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        vector<int> nums = stringToIntegerVector(line);
        getline(cin, line);
        int target = stringToInteger(line);

        int ret = Solution().search(nums, target);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
