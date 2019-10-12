#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> numMovesStones(int a, int b, int c) {
        vector<int> v { a, b, c };
        sort(v.begin(), v.end());
        return vector<int> {
            (v[1] - v[0] == 2 || v[2] - v[1] == 2) ? 1 : (v[1] - v[0] > 1) + (v[2] - v[1] > 1),
            v[2] - v[0] - 2,
        };
    }
};

int stringToInteger(string input) {
    return stoi(input);
}

string integerVectorToString(vector<int> list, int length = -1) {
    if (length == -1) {
        length = list.size();
    }

    if (length == 0) {
        return "[]";
    }

    string result;
    for(int index = 0; index < length; index++) {
        int number = list[index];
        result += to_string(number) + ", ";
    }
    return "[" + result.substr(0, result.length() - 2) + "]";
}

int main() {
    string line;
    while (getline(cin, line)) {
        int a = stringToInteger(line);
        getline(cin, line);
        int b = stringToInteger(line);
        getline(cin, line);
        int c = stringToInteger(line);
        
        vector<int> ret = Solution().numMovesStones(a, b, c);

        string out = integerVectorToString(ret);
        cout << out << endl;
    }
    return 0;
}
