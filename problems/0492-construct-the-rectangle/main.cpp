#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> constructRectangle(int area) {
        for (int w = sqrt(area); ; --w) {
            if (area % w == 0) {
                return vector<int>{ area / w, w };
            }
        }
        return vector<int>();
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
        int area = stringToInteger(line);
        
        vector<int> ret = Solution().constructRectangle(area);

        string out = integerVectorToString(ret);
        cout << out << endl;
    }
    return 0;
}
