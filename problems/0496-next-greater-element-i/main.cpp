#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> nextGreaterElement(vector<int>& nums1, vector<int>& nums2) {
        stack<int> s;
        unordered_map<int, int> um;
        int prev;
            for (int i = 0, size = nums2.size(); i < size; ++i) {
            if (nums2[i] > prev) {
                while (!s.empty() && nums2[i] > nums2[s.top()]) {
                    um[nums2[s.top()]] = nums2[i];
                    s.pop();
                }
            }
            s.push(i);
            prev = nums2[i];
        }
        vector<int> answer(nums1.size());
        for (int i = 0, size = nums1.size(); i < size; ++i) {
            answer[i] = um.find(nums1[i]) == um.end() ? -1 : um[nums1[i]];
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
    string line;
    while (getline(cin, line)) {
        vector<int> nums1 = stringToIntegerVector(line);
        getline(cin, line);
        vector<int> nums2 = stringToIntegerVector(line);
        
        vector<int> ret = Solution().nextGreaterElement(nums1, nums2);

        string out = integerVectorToString(ret);
        cout << out << endl;
    }
    return 0;
}
