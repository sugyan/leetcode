#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int dominantIndex(vector<int>& nums) {
        int m1 = 0, m2 = 0, midx = 0;
        for (int i = 0, size = nums.size(); i < size; ++i) {
            if (nums[i] > m1) {
                midx = i;
                m2 = m1;
                m1 = nums[i];
            } else {
                m2 = max(m2, nums[i]);
            }
        }
        return m1 >= m2 * 2 ? midx : -1;
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
        
        int ret = Solution().dominantIndex(nums);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
