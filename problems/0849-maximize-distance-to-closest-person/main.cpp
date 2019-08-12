#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    int maxDistToClosest(vector<int>& seats) {
        int answer = -1;
        int length = 0;
        for (int i = 0, size = seats.size(); i < size + 1; ++i) {
            if (i < size && seats[i] == 0) ++length;
            else {
                if (answer < 0 || i == size) {
                    answer = max(answer, length);
                } else {
                    answer = max(answer, (length - 1) / 2 + 1);
                }
                length = 0;
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
        vector<int> seats = stringToIntegerVector(line);
        
        int ret = Solution().maxDistToClosest(seats);

        string out = to_string(ret);
        cout << out << endl;
    }
    return 0;
}
