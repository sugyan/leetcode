#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool checkPerfectNumber(int num) {
        if (num < 6) return false;
        int sum = 1;
        for (int i = 2; i <= sqrt(num); ++i) {
            if (num % i != 0) continue;
            sum += i;
            if (i * i != num) sum += num / i;
        }
        return sum == num;
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

string boolToString(bool input) {
    return input ? "True" : "False";
}

int main() {
    string line;
    while (getline(cin, line)) {
        int num = stringToInteger(line);
        
        bool ret = Solution().checkPerfectNumber(num);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
