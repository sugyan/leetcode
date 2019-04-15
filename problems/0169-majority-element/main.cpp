#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int majorityElement(vector<int>& nums) {
        // Boyer–Moore majority vote algorithm
        int count = 0, answer;
        for (int num : nums) {
            if (count == 0) answer = num;
            if (num == answer) {
                ++count;
            } else {
                --count;
            }
        }
        return answer;

        // int size = nums.size();
        // unordered_map<int, int> um;
        // for (int num : nums) {
        //     if (++um[num] > size / 2) {
        //         return num;
        //     }
        // }
        // return 0;
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
        
        int ret = Solution().majorityElement(nums);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
