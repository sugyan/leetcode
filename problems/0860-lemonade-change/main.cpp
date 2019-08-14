#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool lemonadeChange(vector<int>& bills) {
        int b1 = 0, b2 = 0;
        for (int bill : bills) {
            if (bill == 5) {
                ++b1;
            }
            if (bill == 10) {
                if (b1 == 0) return false;
                --b1, ++b2;
            }
            if (bill == 20) {
                if (b1 > 0 && b2 > 0) --b1, --b2;
                else if (b1 >= 3) b1 -= 3;
                else return false;
            }
        }
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
        vector<int> bills = stringToIntegerVector(line);
        
        bool ret = Solution().lemonadeChange(bills);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
