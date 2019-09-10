#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool validMountainArray(vector<int>& A) {
        if (A.size() < 3) return false;
        if (A[0] >= A[1]) return false;
        bool decr = false;
        for (int i = 2, size = A.size(); i < size; ++i) {
            if (decr) {
                if (A[i] >= A[i - 1]) return false;
            } else {
                if (A[i] == A[i - 1]) return false;
                if (A[i] < A[i - 1]) decr = true;
            }
        }
        return decr;
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
        vector<int> A = stringToIntegerVector(line);
        
        bool ret = Solution().validMountainArray(A);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
