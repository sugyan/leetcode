#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int findShortestSubArray(vector<int>& nums) {
        int answer = 0, degree = 0;
        unordered_map<int, int> count, first;
        for (int i = 0, size = nums.size(); i < size; ++i) {
            int c = ++count[nums[i]];
            if (first.find(nums[i]) == first.end()) first[nums[i]] = i;
            if (c > degree) {
                degree = c;
                answer = i - first[nums[i]] + 1;
            } else if (c == degree) {
                answer = min(answer, i - first[nums[i]] + 1);
            }
        }
        return answer;
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
        
        int ret = Solution().findShortestSubArray(nums);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
