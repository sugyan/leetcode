#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int numPairsDivisibleBy60(vector<int>& time) {
        vector<int> v(60, 0);
        for (int n : time) ++v[n % 60];
        int answer = 0;
        for (int i = 0; i <= 30; ++i) {
            if (i % 30 == 0) {
                answer += v[i] * (v[i] - 1) / 2;
            } else {
                answer += v[i] * v[60 - i];
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
        vector<int> time = stringToIntegerVector(line);
        
        int ret = Solution().numPairsDivisibleBy60(time);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
