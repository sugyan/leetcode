#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool isRectangleOverlap(vector<int>& rec1, vector<int>& rec2) {
        if (rec1[0] >= rec2[2] || rec1[2] <= rec2[0]) return false;
        if (rec1[1] >= rec2[3] || rec1[3] <= rec2[1]) return false;
        return true;
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

string boolToString(bool input) {
    return input ? "True" : "False";
}

int main() {
    string line;
    while (getline(cin, line)) {
        vector<int> rec1 = stringToIntegerVector(line);
        getline(cin, line);
        vector<int> rec2 = stringToIntegerVector(line);
        
        bool ret = Solution().isRectangleOverlap(rec1, rec2);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
