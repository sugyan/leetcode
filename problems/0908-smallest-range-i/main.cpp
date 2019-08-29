#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int smallestRangeI(vector<int>& A, int K) {
        int maxv = numeric_limits<int>::min(), minv = numeric_limits<int>::max();
        for (int a : A) {
            maxv = max(a, maxv);
            minv = min(a, minv);
        }
        return max(0, (maxv - minv) - 2 * K);
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

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        vector<int> A = stringToIntegerVector(line);
        getline(cin, line);
        int K = stringToInteger(line);
        
        int ret = Solution().smallestRangeI(A, K);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
