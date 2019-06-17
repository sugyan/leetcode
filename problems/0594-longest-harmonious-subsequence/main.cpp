#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int findLHS(vector<int>& nums) {
        if (nums.empty()) return 0;
        map<int, int> m;
        for (int num : nums) ++m[num];
        int answer = 0;
        pair<int, int> prev { m.rbegin()->first, 0 };
        for (pair<int, int> p : m) {
            if (p.first - prev.first == 1) {
                answer = max(answer, p.second + prev.second);
            }
            prev = p;
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
        
        int ret = Solution().findLHS(nums);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
