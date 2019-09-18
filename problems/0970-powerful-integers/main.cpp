#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> powerfulIntegers(int x, int y, int bound) {
        unordered_set<int> answer;
        for (int i = 1; i < bound; i *= x) {
            for (int j = 1; j < bound; j *= y) {
                if (i + j <= bound) {
                    answer.insert(i + j);
                } else {
                    break;
                }
                if (y == 1) break;
            }
            if (x == 1) break;
        }
        return vector<int>(answer.begin(), answer.end());
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
        int x = stringToInteger(line);
        getline(cin, line);
        int y = stringToInteger(line);
        getline(cin, line);
        int bound = stringToInteger(line);
        
        vector<int> ret = Solution().powerfulIntegers(x, y, bound);

        string out = integerVectorToString(ret);
        cout << out << endl;
    }
    return 0;
}
