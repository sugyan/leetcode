#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    bool divisorGame(int N) {
        return N % 2 == 0;
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
        int N = stringToInteger(line);
        
        bool ret = Solution().divisorGame(N);

        string out = boolToString(ret);
        cout << out << endl;
    }
    return 0;
}
