#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string largestTimeFromDigits(vector<int>& A) {
        int hmax = -1, mmax = -1;
        for (int i = 0; i < 4; ++i) {
            for (int j = 0; j < 4; ++j) {
                if (i == j) continue;
                for (int k = 0; k < 4; ++k) {
                    if (k == i || k == j) continue;
                    int l = 6 - i - j - k;
                    int h = A[i] * 10 + A[j];
                    int m = A[k] * 10 + A[l];
                    if (h < 24 && m < 60) {
                        if (h >= hmax) {
                            if (h > hmax || m > mmax) mmax = m;
                            hmax = h;
                        }
                    }
                }
            }
        }
        if (hmax < 0) return "";
        return
            (hmax < 10 ? "0" : "") + to_string(hmax) +
            ':' +
            (mmax < 10 ? "0" : "") + to_string(mmax);
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
        vector<int> A = stringToIntegerVector(line);
        
        string ret = Solution().largestTimeFromDigits(A);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
