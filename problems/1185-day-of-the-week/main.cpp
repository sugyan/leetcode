#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    string dayOfTheWeek(int day, int month, int year) {
        vector<string> days = { "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday" };
        vector<int> dom { 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 };
        int d = 4;
        for (int i = 1970; i < year; ++i) {
            d += 365;
            if (i % 4 == 0) ++d;
        }
        for (int i = 0; i < month - 1; ++i) {
            d += dom[i];
            if (i == 1 && year % 4 == 0) ++d;
        }
        d += day - 1;
        return days[d % 7];
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

int main() {
    string line;
    while (getline(cin, line)) {
        int day = stringToInteger(line);
        getline(cin, line);
        int month = stringToInteger(line);
        getline(cin, line);
        int year = stringToInteger(line);
        
        string ret = Solution().dayOfTheWeek(day, month, year);

        string out = (ret);
        cout << out << endl;
    }
    return 0;
}
